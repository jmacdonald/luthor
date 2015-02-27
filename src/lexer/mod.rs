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

mod tests {
    use super::new;

    #[test]
    fn new_initializes_correctly() {
        let lexer_data = "lexer data";
        let lexer = new(lexer_data);
        assert_eq!(lexer.data, lexer_data);
        assert_eq!(lexer.char_count, 10);
        assert_eq!(lexer.token_start, 0);
        assert_eq!(lexer.token_position, 0);
        assert_eq!(lexer.tokens, vec![]);
    }
}
