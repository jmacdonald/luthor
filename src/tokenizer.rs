extern crate core;

use std::mem;
use std::cmp::min;
use std::iter::FromIterator;
use self::core::str::next_code_point;
use super::token::Token;
use super::token::Category;

pub struct StateFunction(pub fn(&mut Tokenizer) -> Option<StateFunction>);

/// The Tokenizer type is used to produce and store
/// tokens for the various language and format lexers.
pub struct Tokenizer {
    pub data: String,
    binary_data: Vec<u8>,
    head: usize,
    tail: usize,
    char_count: usize,
    pub token_start: usize,
    pub token_position: usize,
    tokens: Vec<Token>,
    pub states: Vec<StateFunction>,
}

/// Initializes a new tokenizer with the given data.
///
/// # Examples
///
/// ```
/// let lexer = luthor::tokenizer::new("luthor");
/// ```
pub fn new(data: &str) -> Tokenizer {
    Tokenizer{
      data: data.to_string(),
      binary_data: data.to_string().into_bytes(),
      head: 0,
      tail: 0,
      char_count: data.chars().count(),
      token_start: 0,
      token_position: 0,
      tokens: vec![],
      states: vec![]
    }
}

impl Tokenizer {
    /// Returns a copy of the tokens processed to date.
    ///
    /// # Examples
    ///
    /// ```
    /// let lexer = luthor::tokenizer::new("luthor");
    /// lexer.tokens();
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
    /// let mut lexer = luthor::tokenizer::new("luthor");
    /// assert_eq!(lexer.current_char().unwrap(), 'l');
    /// lexer.advance();
    /// assert_eq!(lexer.current_char().unwrap(), 'u');
    /// ```
    pub fn advance(&mut self) {
        if self.has_more_data() {
            // Get the current character so that we can calculate
            // its byte length and advance the head appropriately.
            match self.current_char() {
                Some(c) => {
                    self.token_position += 1;
                    self.head += c.len_utf8();
                },
                None => (),
            }
        }
    }

    /// Determines whether or not there is more unprocessed data.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lexer = luthor::tokenizer::new("l");
    /// assert_eq!(lexer.has_more_data(), true);
    /// lexer.advance();
    /// assert_eq!(lexer.has_more_data(), false);
    /// ```
    pub fn has_more_data(&self) -> bool {
        self.token_position < self.char_count
    }

    /// Returns the character at the current position,
    /// unless all of the data has been processed.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lexer = luthor::tokenizer::new("l");
    /// assert_eq!(lexer.current_char().unwrap(), 'l');
    /// lexer.advance();
    /// assert_eq!(lexer.current_char(), None);
    /// ```
    pub fn current_char(&self) -> Option<char> {
        if self.has_more_data() {
            // Create an iterator for the remaining data.
            let mut remaining_data = self.binary_data[self.head..].iter();

            // Pull the first UTF-8 byte sequence and convert it to a char.
            next_code_point(&mut remaining_data).map(|code_point| {
                unsafe {
                    mem::transmute(code_point)
                }
            })
        } else {
            None
        }
    }

    /// Creates and stores a token with the given category containing any
    /// data processed using `advance` since the last call to this method.
    ///
    /// # Examples
    ///
    /// ```
    /// use luthor::token::Category;
    /// let mut lexer = luthor::tokenizer::new("luthor");
    /// lexer.advance();
    /// lexer.advance();
    /// lexer.tokenize(Category::Text);
    /// assert_eq!(lexer.tokens()[0].lexeme, "lu");
    /// ```
    pub fn tokenize(&mut self, category: Category) {
        if self.head > self.tail {
            // Build the lexeme by slicing the currently
            // selected range out of the buffer.
            let lexeme = unsafe {
                String::from_utf8_unchecked(
                    self.binary_data[self.tail..self.head].to_vec()
                )
            };
            let token = Token{
                lexeme: lexeme,
                category: category,
            };
            self.tokens.push(token);
            self.token_start = self.token_position;
            self.tail = self.head;
        }
    }

    /// Creates and stores a token with the given category and the
    /// next `amount` characters of the data. Before doing this, it
    /// tokenizes any previously processed characters with the generic
    /// Category::Text category.
    ///
    /// # Examples
    ///
    /// ```
    /// use luthor::token::Category;
    /// use luthor::token::Token;
    ///
    /// let mut lexer = luthor::tokenizer::new("luthor");
    /// lexer.advance();
    /// lexer.tokenize_next(5, Category::Keyword);
    /// assert_eq!(lexer.tokens()[0], Token{ lexeme: "l".to_string(), category: Category::Text});
    /// assert_eq!(lexer.tokens()[1], Token{ lexeme: "uthor".to_string(), category: Category::Keyword});
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
    fn new_initializes_correctly_with_unicode_data() {
        let lexer_data = "différent";
        let lexer = new(lexer_data);
        assert_eq!(lexer.data, lexer_data);
        assert_eq!(lexer.char_count, 9);
        assert_eq!(lexer.token_start, 0);
        assert_eq!(lexer.token_position, 0);
        assert_eq!(lexer.tokens, vec![]);
    }

    #[test]
    fn advance_increments_the_cursor_by_one() {
        let lexer_data = "élégant";
        let mut lexer = new(lexer_data);
        lexer.advance();
        assert_eq!(lexer.token_position, 1);
        lexer.advance();
        assert_eq!(lexer.token_position, 2);
    }

    #[test]
    fn advance_stops_when_there_is_no_more_data() {
        let lexer_data = "élégant";
        let mut lexer = new(lexer_data);

        // Try to go beyond the last character.
        for _ in 0..lexer.char_count {
            lexer.advance();
        }

        assert_eq!(lexer.token_position, lexer.char_count);
    }

    #[test]
    fn has_more_data_works() {
        let lexer_data = "él";
        let mut lexer = new(lexer_data);

        lexer.advance();
        assert!(lexer.has_more_data());

        lexer.advance();
        assert_eq!(lexer.has_more_data(), false);
    }

    #[test]
    fn current_char_returns_the_char_at_token_position() {
        let lexer_data = "él";
        let lexer = new(lexer_data);

        assert_eq!(lexer.current_char().unwrap(), 'é');
    }

    #[test]
    fn current_char_returns_none_if_at_the_end() {
        let lexer_data = "él";
        let mut lexer = new(lexer_data);
        lexer.advance();
        lexer.advance();

        assert_eq!(lexer.current_char(), None);
    }

    #[test]
    fn tokenize_advances_token_start_to_cursor() {
        let lexer_data = "élégant";
        let mut lexer = new(lexer_data);
        lexer.advance();
        lexer.advance();
        lexer.tokenize(Category::Text);

        assert_eq!(lexer.token_start, 2);
    }

    #[test]
    fn tokenize_creates_the_correct_token() {
        let lexer_data = "élégant";
        let mut lexer = new(lexer_data);
        lexer.advance();
        lexer.advance();
        lexer.tokenize(Category::Text);

        let token = lexer.tokens.pop().unwrap();
        let expected_token = Token{ lexeme: "él".to_string(), category: Category::Text};
        assert_eq!(token, expected_token);
    }

    #[test]
    fn tokenize_does_nothing_if_range_is_empty() {
        let lexer_data = "élégant";
        let mut lexer = new(lexer_data);
        lexer.tokenize(Category::Text);

        assert_eq!(lexer.tokens.len(), 0);
        assert_eq!(lexer.token_start, 0);
        assert_eq!(lexer.token_position, 0);
    }

    #[test]
    fn tokenize_next_tokenizes_previous_data_as_text() {
        let lexer_data = "élégant";
        let mut lexer = new(lexer_data);
        lexer.advance();
        lexer.advance();
        lexer.tokenize_next(1, Category::Keyword);

        let token = lexer.tokens.remove(0);
        let expected_token = Token{ lexeme: "él".to_string(), category: Category::Text};
        assert_eq!(token, expected_token);
    }

    #[test]
    fn tokenize_next_tokenizes_next_x_chars() {
        let lexer_data = "élégant";
        let mut lexer = new(lexer_data);
        lexer.advance();
        lexer.advance();
        lexer.tokenize_next(5, Category::Keyword);

        let token = lexer.tokens.pop().unwrap();
        let expected_token = Token{ lexeme: "égant".to_string(), category: Category::Keyword};
        assert_eq!(token, expected_token);
    }

    #[test]
    fn tokenize_next_takes_at_most_what_is_left() {
        let lexer_data = "élégant";
        let mut lexer = new(lexer_data);
        lexer.advance();
        lexer.advance();
        lexer.tokenize_next(15, Category::Keyword);

        let token = lexer.tokens.pop().unwrap();
        let expected_token = Token{ lexeme: "égant".to_string(), category: Category::Keyword};
        assert_eq!(token, expected_token);
    }
}
