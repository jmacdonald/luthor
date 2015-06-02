use tokenizer::new;
use tokenizer::Tokenizer;
use tokenizer::StateFunction;
use token::Token;
use token::Category;

fn initial_state(lexer: &mut Tokenizer) -> Option<StateFunction> {
    match lexer.current_char() {
        Some(c) => {
            if lexer.starts_with("</") {
                lexer.tokenize(Category::Identifier);
                lexer.tokenize_next(2, Category::Text);
                return Some(StateFunction(inside_tag))
            }
            match c {
                '<' => {
                    lexer.tokenize_next(1, Category::Text);
                    return Some(StateFunction(start_of_tag));
                },
                ' ' | '\n' => {
                    lexer.tokenize(Category::Text);
                    lexer.advance();
                    lexer.states.push(StateFunction(initial_state));
                    return Some(StateFunction(whitespace));
                },
                _ => {
                    lexer.advance();
                }
            }

            Some(StateFunction(initial_state))
        }

        None => {
            lexer.tokenize(Category::Text);
            None
        }
    }
}

fn start_of_tag(lexer: &mut Tokenizer) -> Option<StateFunction> {
    match lexer.current_char() {
        Some(c) => {
            match c {
                ' ' | '\n' => {
                    lexer.tokenize(Category::Identifier);
                    lexer.states.push(StateFunction(inside_tag));
                    return Some(StateFunction(whitespace));
                },
                '>' => {
                    lexer.tokenize(Category::Identifier);
                    lexer.tokenize_next(1, Category::Text);
                    Some(StateFunction(initial_state))
                }
                _ => {
                    lexer.advance();
                    Some(StateFunction(start_of_tag))
                }
            }
        }

        None => {
            lexer.tokenize(Category::Identifier);
            None
        }
    }
}

fn inside_tag(lexer: &mut Tokenizer) -> Option<StateFunction> {
    match lexer.current_char() {
        Some(c) => {
            match c {
                '"' => {
                    lexer.tokenize(Category::Identifier);
                    lexer.advance();
                    Some(StateFunction(inside_string))
                },
                ' ' | '\n' => {
                    lexer.tokenize(Category::Identifier);
                    lexer.advance();
                    lexer.states.push(StateFunction(inside_tag));
                    return Some(StateFunction(whitespace));
                },
                '=' => {
                    lexer.tokenize(Category::Identifier);
                    lexer.tokenize_next(1, Category::AssignmentOperator);
                    Some(StateFunction(inside_tag))
                }
                '>' => {
                    lexer.tokenize(Category::Identifier);
                    lexer.tokenize_next(1, Category::Text);
                    Some(StateFunction(initial_state))
                }
                _ => {
                    if lexer.starts_with("/>") {
                        lexer.tokenize(Category::Identifier);
                        lexer.tokenize_next(2, Category::Text);
                        return Some(StateFunction(initial_state))
                    }

                    lexer.advance();
                    Some(StateFunction(inside_tag))
                }
            }
        }

        None => {
            lexer.tokenize(Category::Identifier);
            None
        }
    }
}

fn inside_string(lexer: &mut Tokenizer) -> Option<StateFunction> {
    match lexer.current_char() {
        Some(c) => {
            match c {
                '"' => {
                    lexer.advance();
                    lexer.tokenize(Category::String);
                    Some(StateFunction(inside_tag))
                },
                '\\' => {
                    lexer.advance();
                    lexer.advance();
                    Some(StateFunction(inside_string))
                }
                _ => {
                    lexer.advance();
                    Some(StateFunction(inside_string))
                }
            }
        }

        None => {
            lexer.tokenize(Category::String);
            None
        }
    }
}

fn whitespace(lexer: &mut Tokenizer) -> Option<StateFunction> {
    match lexer.current_char() {
        Some(c) => {
            match c {
                ' ' | '\n' => {
                    lexer.advance();
                    Some(StateFunction(whitespace))
                },
                _ => {
                    lexer.tokenize(Category::Whitespace);
                    Some(lexer.states.pop().unwrap())
                }
            }
        }

        None => {
            lexer.tokenize(Category::Whitespace);
            None
        }
    }
}

pub fn lex(data: &str) -> Vec<Token> {
    let mut lexer = new(data);
    let mut state_function = StateFunction(initial_state);
    loop {
        let StateFunction(actual_function) = state_function;
        match actual_function(&mut lexer) {
            Some(f) => state_function = f,
            None => return lexer.tokens(),
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
