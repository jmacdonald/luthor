pub use self::token::Token;
pub use self::token::Category;

pub mod token;
pub mod formats;

pub struct StateFunction {
    f: fn(&str, &mut Vec<Token>) -> Option<StateFunction>,
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
            self.token_position += 1;
            let potential_token = self.data.slice(self.token_start, self.token_position);
            match (self.state_function.f)(potential_token, &mut self.tokens) {
                Some(f) => {
                    self.state_function = f;

                    match self.tokens.pop() {
                        Some(t) => {
                            self.token_start = self.token_position;
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
    use super::formats;
    use super::Token;
    use super::Category;
    use super::StateFunction;

    #[test]
    fn it_works() {
        let mut lexer = new("{ \"villain\": \"luthor\" }", StateFunction{ f: formats::json::initial_state });
        let mut tokens = vec![];
        for token in lexer {
            tokens.push(token);
        }

        assert_eq!(tokens, vec![Token{ lexeme: "{".to_string(), category: Category::Brace }]);
    }
}
