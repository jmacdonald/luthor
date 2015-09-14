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
    /// Returns a copy of the tokens processed to date, in addition to any
    /// in-progress or remaining data appended as a text-category token.
    /// As a result, the returned tokens always produce the original dataset
    /// when their lexemes are concatenated.
    ///
    /// # Examples
    ///
    /// ```
    /// use luthor::token::{Category, Token};
    ///
    /// // Set up a new tokenizer.
    /// let mut tokenizer = luthor::tokenizer::new("luthor");
    /// tokenizer.tokenize_next(2, Category::Keyword);
    ///
    /// assert_eq!(
    ///     tokenizer.tokens(),
    ///     vec![
    ///         Token{ lexeme: "lu".to_string(), category: Category::Keyword },
    ///         Token{ lexeme: "thor".to_string(), category: Category::Text }
    ///     ]
    /// );
    ///
    /// ```
    pub fn tokens(&self) -> Vec<Token> {
        let mut tokens = self.tokens.clone();

        // Duplicate the tokenizer's character iterator so that we can
        // advance it to check for equality without affecting the original.
        let data_iter = self.data.clone();

        // Append any remaining data to the in-progress token.
        let mut remaining_data = self.current_token.clone();
        for c in data_iter {
            remaining_data.push(c);
        }
            
        // If there was any remaining or in-progress data, add it as a text token.
        if !remaining_data.is_empty() {
            tokens.push(Token{ lexeme: remaining_data, category: Category::Text});
        }

        tokens
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

    /// Returns the next non-whitespace character, without advancing the cursor.
    ///
    /// # Examples
    ///
    /// ```
    /// // Set up a new tokenizer.
    /// let mut tokenizer = luthor::tokenizer::new("  b");
    ///
    /// // Ask for the next non-whitespace character.
    /// assert_eq!(tokenizer.next_non_whitespace_char().unwrap(), 'b');
    ///
    /// // Advance past the "b" character and ask again.
    /// for _ in 0..3 { tokenizer.advance(); }
    /// assert!(tokenizer.next_non_whitespace_char().is_none());
    ///
    /// ```
    pub fn next_non_whitespace_char(&self) -> Option<char> {
        // Duplicate the tokenizer's character iterator so that we can
        // advance it to check for equality without affecting the original.
        let mut data_iter = self.data.clone();

        data_iter.find(|&c| c != ' ' && c != '\n')
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
    /// Ensures that the specified lexeme is not just a prefix by checking that
    /// the data that follows it is a newline, space, comma, or nothing at all.
    ///
    /// # Examples
    ///
    /// ```
    /// use luthor::token::Category;
    ///
    /// // Set up a new tokenizer.
    /// let mut tokenizer = luthor::tokenizer::new("lex\nluthor lib,rary");
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
    /// // Commas delineate lexemes.
    /// assert!(tokenizer.starts_with_lexeme("lib"));
    ///
    /// // Consume 4 characters, advancing to the next lexeme.
    /// tokenizer.tokenize_next(4, Category::Text);
    ///
    /// // End of string delineates lexemes.
    /// assert!(tokenizer.starts_with_lexeme("rary"));
    /// ```
    pub fn starts_with_lexeme(&self, lexeme: &str) -> bool {
        // Duplicate the tokenizer's character iterator so that we can
        // advance it to check for equality without affecting the original.
        let data_iter = self.data.clone();

        self.has_prefix(lexeme) && match data_iter.skip(lexeme.len()).next() {
            Some(' ') | Some('\n') | Some(',') => true,
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

    /// Consumes consecutive whitespace characters as a single token.
    ///
    /// # Examples
    ///
    /// ```
    /// use luthor::token::Category;
    /// use luthor::token::Token;
    ///
    /// let mut tokenizer = luthor::tokenizer::new("  \nluthor");
    /// tokenizer.consume_whitespace();
    ///
    /// assert_eq!(
    ///     tokenizer.tokens()[0],
    ///     Token{ lexeme: "  \n".to_string(), category: Category::Whitespace }
    /// );
    /// ```
    pub fn consume_whitespace(&mut self) {
        let mut found_whitespace = false;
        loop {
            match self.current_char() {
                Some(' ') | Some('\n') => {
                    if !found_whitespace {
                        self.tokenize(Category::Text);
                        found_whitespace = true;
                    }

                    self.advance();
                },
                _ => {
                    if found_whitespace {
                        self.tokenize(Category::Whitespace);
                    }
                    return
                }
            }
        }
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

    #[test]
    fn consume_whitespace_handles_preexisting_noncategorized_chars() {
        let data = "e  ";
        let mut tokenizer = new(data);
        tokenizer.advance();
        tokenizer.consume_whitespace();

        assert_eq!(
            tokenizer.tokens()[0],
            Token{ lexeme: "e".to_string(), category: Category::Text }
        );
        assert_eq!(
            tokenizer.tokens()[1],
            Token{ lexeme: "  ".to_string(), category: Category::Whitespace }
        );
    }

    #[test]
    fn tokens_returns_unprocessed_data_as_text_token() {
        let tokenizer = new("luthor");

        assert_eq!(
            tokenizer.tokens()[0],
            Token{ lexeme: "luthor".to_string(), category: Category::Text }
        );
    }

    #[test]
    fn tokens_joins_advanced_data_with_unprocessed_data_as_text_token() {
        let mut tokenizer = new("luthor");
        tokenizer.advance();

        assert_eq!(
            tokenizer.tokens()[0],
            Token{ lexeme: "luthor".to_string(), category: Category::Text }
        );
    }
}
