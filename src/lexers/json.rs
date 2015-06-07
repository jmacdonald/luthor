use tokenizer::new;
use tokenizer::Tokenizer;
use tokenizer::StateFunction;
use token::Token;
use token::Category;

fn initial_state(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            match c {
                '{' => {
                    tokenizer.tokenize_next(1, Category::Brace);
                },
                '[' => {
                    tokenizer.tokenize_next(1, Category::Bracket);
                },
                ' ' | '\n' => {
                    tokenizer.tokenize(Category::Text);
                    tokenizer.advance();
                    return Some(StateFunction(whitespace));
                },
                '"' => {
                    tokenizer.tokenize(Category::Text);
                    tokenizer.advance();
                    return Some(StateFunction(inside_string));
                },
                ':' => {
                    tokenizer.tokenize_next(1, Category::Operator);
                },
                '}' => {
                    tokenizer.tokenize_next(1, Category::Brace);
                },
                ']' => {
                    tokenizer.tokenize_next(1, Category::Bracket);
                },
                _ => {
                    if tokenizer.starts_with("true") {
                        tokenizer.tokenize_next(4, Category::Boolean);
                    } else if tokenizer.starts_with("false") {
                        tokenizer.tokenize_next(5, Category::Boolean);
                    } else if tokenizer.starts_with("null") {
                        tokenizer.tokenize_next(4, Category::Keyword);
                    } else {
                        tokenizer.advance();
                    }
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
                    Some(StateFunction(initial_state))
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
        let data = include_str!("../../test_data/data.json");
        let tokens = lex(data);
        let expected_tokens = vec![
            Token{ lexeme: "{".to_string(), category: Category::Brace },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"key\"".to_string(), category: Category::String },
            Token{ lexeme: ":".to_string(), category: Category::Operator },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"4032\"".to_string(), category: Category::String },
            Token{ lexeme: ",".to_string(), category: Category::Text },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "'single'".to_string(), category: Category::Text },
            Token{ lexeme: ":".to_string(), category: Category::Operator },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "'quotes\\'',".to_string(), category: Category::Text },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"literals\"".to_string(), category: Category::String },
            Token{ lexeme: ":".to_string(), category: Category::Operator },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "[".to_string(), category: Category::Bracket },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "true".to_string(), category: Category::Boolean },
            Token{ lexeme: ",".to_string(), category: Category::Text },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "false".to_string(), category: Category::Boolean },
            Token{ lexeme: ",".to_string(), category: Category::Text },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "null".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "]".to_string(), category: Category::Bracket },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "}".to_string(), category: Category::Brace },
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
            Token{ lexeme: "}".to_string(), category: Category::Brace },
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
        let tokens = lex("\"open!");
        let expected_tokens = vec![
            Token{ lexeme: "\"open!".to_string(), category: Category::String },
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
