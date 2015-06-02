use tokenizer::new;
use tokenizer::Tokenizer;
use tokenizer::StateFunction;
use token::Token;
use token::Category;

fn initial_state(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            if tokenizer.starts_with("</") {
                tokenizer.tokenize(Category::Identifier);
                tokenizer.tokenize_next(2, Category::Text);
                return Some(StateFunction(inside_tag))
            }
            match c {
                '<' => {
                    tokenizer.tokenize_next(1, Category::Text);
                    return Some(StateFunction(start_of_tag));
                },
                ' ' | '\n' => {
                    tokenizer.tokenize(Category::Text);
                    tokenizer.advance();
                    tokenizer.states.push(StateFunction(initial_state));
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

fn start_of_tag(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            match c {
                ' ' | '\n' => {
                    tokenizer.tokenize(Category::Identifier);
                    tokenizer.states.push(StateFunction(inside_tag));
                    return Some(StateFunction(whitespace));
                },
                '>' => {
                    tokenizer.tokenize(Category::Identifier);
                    tokenizer.tokenize_next(1, Category::Text);
                    Some(StateFunction(initial_state))
                }
                _ => {
                    tokenizer.advance();
                    Some(StateFunction(start_of_tag))
                }
            }
        }

        None => {
            tokenizer.tokenize(Category::Identifier);
            None
        }
    }
}

fn inside_tag(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            match c {
                '"' => {
                    tokenizer.tokenize(Category::Identifier);
                    tokenizer.advance();
                    Some(StateFunction(inside_string))
                },
                ' ' | '\n' => {
                    tokenizer.tokenize(Category::Identifier);
                    tokenizer.advance();
                    tokenizer.states.push(StateFunction(inside_tag));
                    return Some(StateFunction(whitespace));
                },
                '=' => {
                    tokenizer.tokenize(Category::Identifier);
                    tokenizer.tokenize_next(1, Category::AssignmentOperator);
                    Some(StateFunction(inside_tag))
                }
                '>' => {
                    tokenizer.tokenize(Category::Identifier);
                    tokenizer.tokenize_next(1, Category::Text);
                    Some(StateFunction(initial_state))
                }
                _ => {
                    if tokenizer.starts_with("/>") {
                        tokenizer.tokenize(Category::Identifier);
                        tokenizer.tokenize_next(2, Category::Text);
                        return Some(StateFunction(initial_state))
                    }

                    tokenizer.advance();
                    Some(StateFunction(inside_tag))
                }
            }
        }

        None => {
            tokenizer.tokenize(Category::Identifier);
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
                    Some(StateFunction(inside_tag))
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
        let data = include_str!("../../test_data/data.xml");
        let tokens = lex(data);
        let expected_tokens = vec![
            Token{ lexeme: "<".to_string(), category: Category::Text },
            Token{ lexeme: "tag".to_string(), category: Category::Identifier },
            Token{ lexeme: ">".to_string(), category: Category::Text },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "<".to_string(), category: Category::Text },
            Token{ lexeme: "tag_with_attribute".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "attribute".to_string(), category: Category::Identifier },
            Token{ lexeme: "=".to_string(), category: Category::AssignmentOperator },
            Token{ lexeme: "\"value\"".to_string(), category: Category::String },
            Token{ lexeme: ">".to_string(), category: Category::Text },
            Token{ lexeme: "</".to_string(), category: Category::Text },
            Token{ lexeme: "tag_with_attribute".to_string(), category: Category::Identifier },
            Token{ lexeme: ">".to_string(), category: Category::Text },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "<".to_string(), category: Category::Text },
            Token{ lexeme: "self_closing_tag".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "/>".to_string(), category: Category::Text },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "</".to_string(), category: Category::Text },
            Token{ lexeme: "tag".to_string(), category: Category::Identifier },
            Token{ lexeme: ">".to_string(), category: Category::Text },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
        ];

        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[index]);
        }
    }

    #[test]
    fn it_can_handle_garbage() {
        let tokens = lex("} adwyx123&*_ ");
        let expected_tokens = vec![
            Token{ lexeme: "}".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "adwyx123&*_".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
        ];

        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[index]);
        }
    }

    #[test]
    fn it_can_handle_open_strings() {
        let tokens = lex("<tag \"open!>");
        let expected_tokens = vec![
            Token{ lexeme: "<".to_string(), category: Category::Text },
            Token{ lexeme: "tag".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"open!>".to_string(), category: Category::String },
        ];

        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[index]);
        }
    }

    #[test]
    fn it_can_handle_utf8_data() {
        let tokens = lex("différent");
        let expected_tokens = vec![
            Token{ lexeme: "différent".to_string(), category: Category::Text },
        ];

        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[index]);
        }
    }
}
