//! Utility module for lexer implementations,
//! providing types to help manage states and tokens.

use std::str::Chars;
use super::token::Token;
use super::token::Category;

/// A recursive function type used by lexers to manage their state.
/// Based on Rob Pike's "Lexical Scanning in Go" talk, these functions are
/// invoked in a call/return loop (letting the current function determine
/// the next) until a `None` value is returned, after which lexing is complete.
///
/// See the `lexers` module for examples.
pub struct StateFunction(pub fn(&mut Tokenizer) -> Option<StateFunction>);

/// The Tokenizer type is used to produce and store tokens for lexers.
pub struct Tokenizer<'a> {
    data: Chars<'a>,
    current_token: String,
    tokens: Vec<Token>,
    pub states: Vec<StateFunction>,
}

/// Initializes a new tokenizer with the given data.
///
/// # Examples
///
/// ```
/// let tokenizer = luthor::tokenizer::new("luthor");
/// ```
pub fn new(data: &str) -> Tokenizer {
    Tokenizer{
      data: data.chars(),
      current_token: String::new(),
      tokens: vec![],
      states: vec![]
    }
}

impl<'a> Tokenizer<'a> {
    /// Returns a copy of the tokens processed to date.
    ///
    /// # Examples
    ///
    /// ```
    /// // Set up a new tokenizer.
    /// let tokenizer = luthor::tokenizer::new("luthor");
    ///
    /// tokenizer.tokens();
    /// ```
    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    /// Moves to the next character in the data.
    /// Does nothing if there is no more data to process.
    ///
    /// # Examples
    ///
    /// ```
    /// // Set up a new tokenizer.
    /// let mut tokenizer = luthor::tokenizer::new("luthor");
    ///
    /// // Ensure that we're at the first character. 
    /// assert_eq!(tokenizer.current_char().unwrap(), 'l');
    ///
    /// // Consume the first character.
    /// tokenizer.advance();
    ///
    /// // Ensure that we're at the next character.
    /// assert_eq!(tokenizer.current_char().unwrap(), 'u');
    /// ```
    pub fn advance(&mut self) {
        match self.data.next() {
            Some(c) => self.current_token.push(c),
            None => ()
        }
    }

    /// Returns the character at the current position,
    /// unless all of the data has been processed.
    ///
    /// # Examples
    ///
    /// ```
    /// // Set up a new tokenizer.
    /// let mut tokenizer = luthor::tokenizer::new("l");
    ///
    /// // Ensure that the current character is correct.
    /// assert_eq!(tokenizer.current_char().unwrap(), 'l');
    ///
    /// // Consume the last bit of data.
    /// tokenizer.advance();
    ///
    /// // Ensure that there is no current character.
    /// assert_eq!(tokenizer.current_char(), None);
    /// ```
    pub fn current_char(&self) -> Option<char> {
        match self.data.clone().peekable().peek() {
            Some(c) => Some(c.clone()),
            None => None
        }
    }


    /// Whether or not the remaining data starts with the specified prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// // Set up a new tokenizer.
    /// let tokenizer = luthor::tokenizer::new("lex");
    ///
    /// assert!(tokenizer.has_prefix("le"));
    /// ```
    pub fn has_prefix(&self, prefix: &str) -> bool {
        // Duplicate the tokenizer's character iterator so that we can
        // advance it to check for equality without affecting the original.
        let mut data_iter = self.data.clone();

        // Check that the subject is a prefix, character by character.
        // This is much faster than building a string of equal length from
        // self.data and deferring to a straight string comparison using ==.
        prefix.chars().all(|c| {
            match data_iter.next() {
                Some(d) => c == d,
                None => false
            }
        })
    }

    /// Whether or not the remaining data starts with the specified lexeme.
    /// Ensures that the specified lexeme is not just a prefix by checking
    /// that the data that follows it is a newline, space, or nothing at all.
    ///
    /// # Examples
    ///
    /// ```
    /// use luthor::token::Category;
    ///
    /// // Set up a new tokenizer.
    /// let mut tokenizer = luthor::tokenizer::new("lex\nluthor lib");
    ///
    /// // Prefixes don't count.
    /// assert!(!tokenizer.starts_with_lexeme("le"));
    ///
    /// // Newlines delineate lexemes.
    /// assert!(tokenizer.starts_with_lexeme("lex"));
    ///
    /// // Consume 4 characters, advancing to the next lexeme.
    /// tokenizer.tokenize_next(4, Category::Text);
    ///
    /// // Spaces delineate lexemes.
    /// assert!(tokenizer.starts_with_lexeme("luthor"));
    ///
    /// // Consume 7 characters, advancing to the next lexeme.
    /// tokenizer.tokenize_next(7, Category::Text);
    ///
    /// // End of string delineates lexemes.
    /// assert!(tokenizer.starts_with_lexeme("lib"));
    /// ```
    pub fn starts_with_lexeme(&self, lexeme: &str) -> bool {
        // Duplicate the tokenizer's character iterator so that we can
        // advance it to check for equality without affecting the original.
        let data_iter = self.data.clone();

        self.has_prefix(lexeme) && match data_iter.skip(lexeme.len()).next() {
            Some(' ') | Some('\n') => true,
            None => true,
            _ => false
        }
    }

    /// Creates and stores a token with the given category containing any
    /// data processed using `advance` since the last call to this method.
    ///
    /// # Examples
    ///
    /// ```
    /// use luthor::token::Category;
    ///
    /// // Set up a new tokenizer.
    /// let mut tokenizer = luthor::tokenizer::new("luthor");
    ///
    /// // Consume two characters and then tokenize them.
    /// tokenizer.advance();
    /// tokenizer.advance();
    /// tokenizer.tokenize(Category::Text);
    ///
    /// // Ensure that we have a correctly-categorized token.
    /// assert_eq!(tokenizer.tokens()[0].lexeme, "lu");
    /// ```
    pub fn tokenize(&mut self, category: Category) {
        if !self.current_token.is_empty() {
            let token = Token{
                lexeme: self.current_token.clone(),
                category: category,
            };
            self.tokens.push(token);
            self.current_token = String::new();
        }
    }

    /// Creates and stores a token with the given category and the
    /// next `amount` characters of the data. Before doing this, it
    /// tokenizes any previously processed characters with the generic
    /// `Category::Text` category.
    ///
    /// # Examples
    ///
    /// ```
    /// use luthor::token::Category;
    /// use luthor::token::Token;
    ///
    /// // Set up a new tokenizer.
    /// let mut tokenizer = luthor::tokenizer::new("luthor");
    ///
    /// // Consume one character, and then tokenize the next 5.
    /// tokenizer.advance();
    /// tokenizer.tokenize_next(5, Category::Keyword);
    ///
    /// // Ensure that we have two properly-categorized tokens.
    /// assert_eq!(
    ///     tokenizer.tokens()[0],
    ///     Token{ lexeme: "l".to_string(), category: Category::Text }
    /// );
    /// assert_eq!(
    ///     tokenizer.tokens()[1],
    ///     Token{ lexeme: "uthor".to_string(), category: Category::Keyword }
    /// );
    /// ```
    pub fn tokenize_next(&mut self, amount: usize, category: Category) {
        // If there's any data that has yet
        // to be tokenized, take care of that.
        self.tokenize(Category::Text);

        // Mark the next series of characters.
        for _ in 0..amount { self.advance(); }

        // Tokenize the marked characters.
        self.tokenize(category);
    }
}

#[cfg(test)]
mod tests {
    use super::new;
    use super::super::token::Token;
    use super::super::token::Category;

    #[test]
    fn current_char_returns_the_char_at_head() {
        let data = "él";
        let tokenizer = new(data);

        assert_eq!(tokenizer.current_char().unwrap(), 'é');
    }

    #[test]
    fn current_char_returns_none_if_at_the_end() {
        let data = "él";
        let mut tokenizer = new(data);
        tokenizer.advance();
        tokenizer.advance();

        assert_eq!(tokenizer.current_char(), None);
    }

    #[test]
    fn tokenize_creates_the_correct_token() {
        let data = "élégant";
        let mut tokenizer = new(data);
        tokenizer.advance();
        tokenizer.advance();
        tokenizer.tokenize(Category::Text);

        let token = tokenizer.tokens.pop().unwrap();
        let expected_token = Token{ lexeme: "él".to_string(), category: Category::Text};
        assert_eq!(token, expected_token);
    }

    #[test]
    fn tokenize_does_nothing_if_range_is_empty() {
        let data = "élégant";
        let mut tokenizer = new(data);
        tokenizer.tokenize(Category::Text);

        assert_eq!(tokenizer.tokens.len(), 0);
    }

    #[test]
    fn tokenize_next_tokenizes_previous_data_as_text() {
        let data = "élégant";
        let mut tokenizer = new(data);
        tokenizer.advance();
        tokenizer.advance();
        tokenizer.tokenize_next(1, Category::Keyword);

        let token = tokenizer.tokens.remove(0);
        let expected_token = Token{ lexeme: "él".to_string(), category: Category::Text};
        assert_eq!(token, expected_token);
    }

    #[test]
    fn tokenize_next_tokenizes_next_x_chars() {
        let data = "élégant";
        let mut tokenizer = new(data);
        tokenizer.advance();
        tokenizer.advance();
        tokenizer.tokenize_next(5, Category::Keyword);

        let token = tokenizer.tokens.pop().unwrap();
        let expected_token = Token{ lexeme: "égant".to_string(), category: Category::Keyword};
        assert_eq!(token, expected_token);
    }

    #[test]
    fn tokenize_next_takes_at_most_what_is_left() {
        let data = "élégant";
        let mut tokenizer = new(data);
        tokenizer.advance();
        tokenizer.advance();
        tokenizer.tokenize_next(15, Category::Keyword);

        let token = tokenizer.tokens.pop().unwrap();
        let expected_token = Token{ lexeme: "égant".to_string(), category: Category::Keyword};
        assert_eq!(token, expected_token);
    }
}
