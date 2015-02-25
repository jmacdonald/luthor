pub use self::token::Token;
pub use self::token::Category;

pub mod token;
pub mod implementations;

pub struct StateFunction {
    f: fn(&mut Lexer) -> Option<StateFunction>,
}

pub struct Lexer {
    data: String,
    token_start: usize,
    token_position: usize,
    state_function: StateFunction,
    tokens: Vec<Token>,
}

impl Iterator for Lexer {
    type Item = token::Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            match (self.state_function.f)(self) {
                Some(f) => {
                    self.state_function = f;

                    match self.tokens.pop() {
                        Some(t) => {
                            // A token has been generated. Move the
                            // start and position indices past its
                            // position, and return the token.
                            self.token_start += t.lexeme.len();
                            self.token_position = self.token_start;
                            return Some(t)
                        },
                        None => continue,
                    }
                },
                None => return None,
            }
        }
    }
}

pub fn new(data: &str, initial_state: StateFunction) -> Lexer {
    Lexer{ data: data.to_string(), token_start: 0,
      token_position: 0, state_function: initial_state, tokens: vec![] }
}

mod tests {
    use super::new;
    use super::implementations;
    use super::Token;
    use super::Category;
    use super::StateFunction;

    #[test]
    fn it_works() {
        let mut lexer = new("{ \"villain\": \"luthor\" }", StateFunction{ f: implementations::json::initial_state });
        let mut tokens = vec![];
        for token in lexer {
            tokens.push(token);
        }

        assert_eq!(tokens, vec![Token{ lexeme: "{".to_string(), category: Category::Brace }]);
    }
}
