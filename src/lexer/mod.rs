pub use self::token::Token;
pub use self::token::Category;

pub mod token;
pub mod implementations;

pub struct StateFunction(fn(&mut Lexer) -> Option<StateFunction>);

pub struct Lexer {
    data: String,
    char_count: usize,
    token_start: usize,
    token_position: usize,
    tokens: Vec<Token>,
}

pub fn new(data: &str) -> Lexer {
    Lexer{
      data: data.to_string(),
      char_count: data.chars().count(),
      token_start: 0,
      token_position: 0,
      tokens: vec![]
    }
}

impl Lexer {
    pub fn advance(&mut self) {
        if self.token_position < self.char_count-1 {
            self.token_position += 1;
        }
    }
}

mod tests {
    use super::new;

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

        assert_eq!(lexer.token_position, lexer.char_count-1);
    }
}
