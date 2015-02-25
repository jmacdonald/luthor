pub use self::token::Token;
pub use self::token::Category;

pub mod token;
pub mod implementations;

pub struct StateFunction(fn(&mut Lexer) -> Option<StateFunction>);

pub struct Lexer {
    data: String,
    token_start: usize,
    token_position: usize,
    tokens: Vec<Token>,
}

pub fn new(data: &str) -> Lexer {
    Lexer{
      data: data.to_string(),
      token_start: 0,
      token_position: 0,
      tokens: vec![]
    }
}
