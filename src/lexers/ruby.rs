use tokenizer::new;
use tokenizer::Tokenizer;
use tokenizer::StateFunction;
use token::Token;
use token::Category;

fn initial_state(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            if tokenizer.starts_with("class ") {
                tokenizer.tokenize(Category::Text);
                tokenizer.tokenize_next(5, Category::Keyword);
                tokenizer.states.push(StateFunction(identifier));
                return Some(StateFunction(whitespace))
            } else if tokenizer.starts_with("def ") {
                tokenizer.tokenize(Category::Text);
                tokenizer.tokenize_next(3, Category::Keyword);
                tokenizer.states.push(StateFunction(method));
                return Some(StateFunction(whitespace))
            } else if tokenizer.starts_with("do ") {
                tokenizer.tokenize(Category::Text);
                tokenizer.tokenize_next(2, Category::Keyword);
                tokenizer.states.push(StateFunction(initial_state));
                return Some(StateFunction(whitespace))
            } else if tokenizer.starts_with("end\n") {
                tokenizer.tokenize(Category::Text);
                tokenizer.tokenize_next(3, Category::Keyword);
                tokenizer.states.push(StateFunction(initial_state));
                return Some(StateFunction(whitespace))
            }

            match c {
                '"' => {
                    tokenizer.tokenize(Category::Text);
                    tokenizer.advance();
                    return Some(StateFunction(inside_string));
                },
                '\'' => {
                    tokenizer.tokenize(Category::Text);
                    tokenizer.advance();
                    return Some(StateFunction(inside_single_quote_string));
                },
                ' ' | '\n' => {
                    tokenizer.tokenize(Category::Text);
                    tokenizer.advance();
                    tokenizer.states.push(StateFunction(initial_state));
                    return Some(StateFunction(whitespace));
                },
                '#' => {
                    tokenizer.tokenize(Category::Text);
                    tokenizer.advance();
                    tokenizer.states.push(StateFunction(initial_state));
                    return Some(StateFunction(comment));
                },
                '|' => {
                    tokenizer.tokenize(Category::Text);
                    tokenizer.tokenize_next(1, Category::Text);
                    tokenizer.states.push(StateFunction(identifier));
                    return Some(StateFunction(whitespace));
                },
                _ => {
                    tokenizer.advance();
                }
            }

            Some(StateFunction(initial_state))
        }

        None => {
            tokenizer.tokenize(Category::Text);
            None
        }
    }
}


fn inside_string(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            match c {
                '"' => {
                    tokenizer.advance();
                    tokenizer.tokenize(Category::String);
                    Some(StateFunction(initial_state))
                },
                '\\' => {
                    tokenizer.advance();
                    tokenizer.advance();
                    Some(StateFunction(inside_string))
                }
                _ => {
                    tokenizer.advance();
                    Some(StateFunction(inside_string))
                }
            }
        }

        None => {
            tokenizer.tokenize(Category::String);
            None
        }
    }
}

fn inside_single_quote_string(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            match c {
                '\'' => {
                    tokenizer.advance();
                    tokenizer.tokenize(Category::String);
                    Some(StateFunction(initial_state))
                },
                '\\' => {
                    tokenizer.advance();
                    tokenizer.advance();
                    Some(StateFunction(inside_single_quote_string))
                }
                _ => {
                    tokenizer.advance();
                    Some(StateFunction(inside_single_quote_string))
                }
            }
        }

        None => {
            tokenizer.tokenize(Category::String);
            None
        }
    }
}

fn whitespace(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            match c {
                ' ' | '\n' => {
                    tokenizer.advance();
                    Some(StateFunction(whitespace))
                },
                _ => {
                    tokenizer.tokenize(Category::Whitespace);
                    Some(tokenizer.states.pop().unwrap())
                }
            }
        }

        None => {
            tokenizer.tokenize(Category::Whitespace);
            None
        }
    }
}

fn identifier(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            match c {
                ' ' | '\n' => {
                    tokenizer.tokenize(Category::Identifier);
                    Some(StateFunction(initial_state))
                },
                '|' => {
                    tokenizer.tokenize(Category::Identifier);
                    tokenizer.tokenize_next(1, Category::Text);
                    Some(StateFunction(initial_state))
                },
                _ => {
                    tokenizer.advance();
                    Some(StateFunction(identifier))
                }
            }
        }

        None => {
            tokenizer.tokenize(Category::Whitespace);
            None
        }
    }
}

fn method(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            match c {
                ' ' | '\n' => {
                    tokenizer.tokenize(Category::Method);
                    Some(StateFunction(initial_state))
                },
                _ => {
                    tokenizer.advance();
                    Some(StateFunction(method))
                }
            }
        }

        None => {
            tokenizer.tokenize(Category::Whitespace);
            None
        }
    }
}

fn comment(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            match c {
                '\n' => {
                    tokenizer.tokenize(Category::Comment);
                    Some(StateFunction(initial_state))
                },
                _ => {
                    tokenizer.advance();
                    Some(StateFunction(comment))
                }
            }
        }

        None => {
            tokenizer.tokenize(Category::Whitespace);
            None
        }
    }
}

pub fn lex(data: &str) -> Vec<Token> {
    let mut tokenizer = new(data);
    let mut state_function = StateFunction(initial_state);
    loop {
        let StateFunction(actual_function) = state_function;
        match actual_function(&mut tokenizer) {
            Some(f) => state_function = f,
            None => return tokenizer.tokens(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::lex;
    use token::Token;
    use token::Category;

    #[test]
    fn it_works() {
        let data = include_str!("../../test_data/ruby.rb");
        let tokens = lex(data);
        let expected_tokens = vec![
            Token{ lexeme: "class".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "Ruby".to_string(), category: Category::Identifier },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "def".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "method".to_string(), category: Category::Method },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "# comment".to_string(), category: Category::Comment },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "[".to_string(), category: Category::Text },
            Token{ lexeme: "\"ruby\"".to_string(), category: Category::String },
            Token{ lexeme: "].each".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "do".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "|".to_string(), category: Category::Text },
            Token{ lexeme: "string".to_string(), category: Category::Identifier },
            Token{ lexeme: "|".to_string(), category: Category::Text },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "'string'".to_string(), category: Category::String },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "end".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "end".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "end".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace }
        ];

        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[index]);
        }
    }
}
