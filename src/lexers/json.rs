use tokenizer::new;
use tokenizer::Tokenizer;
use tokenizer::StateFunction;
use token::Token;
use token::Category;

fn initial_state(lexer: &mut Tokenizer) -> Option<StateFunction> {
    match lexer.current_char() {
        Some(c) => {
            match c {
                '{' => {
                    lexer.tokenize_next(1, Category::Brace);
                },
                '[' => {
                    lexer.tokenize_next(1, Category::Bracket);
                },
                ' ' | '\n' => {
                    lexer.tokenize(Category::Text);
                    lexer.advance();
                    return Some(StateFunction(whitespace));
                },
                '"' => {
                    lexer.tokenize(Category::Text);
                    lexer.advance();
                    return Some(StateFunction(inside_string));
                },
                ':' => {
                    lexer.tokenize_next(1, Category::AssignmentOperator);
                },
                '}' => {
                    lexer.tokenize_next(1, Category::Brace);
                },
                ']' => {
                    lexer.tokenize_next(1, Category::Bracket);
                },
                _ => {
                    if lexer.token_position == lexer.token_start {
                        let remaining_data =
                            lexer.data[lexer.token_position..].to_string();

                        if remaining_data.starts_with("true") {
                            lexer.tokenize_next(4, Category::Boolean);
                        } else if remaining_data.starts_with("false") {
                            lexer.tokenize_next(5, Category::Boolean);
                        } else if remaining_data.starts_with("null") {
                            lexer.tokenize_next(4, Category::Keyword);
                        } else {
                            lexer.advance();
                        }
                    } else {
                        lexer.advance();
                    }
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

fn inside_string(lexer: &mut Tokenizer) -> Option<StateFunction> {
    match lexer.current_char() {
        Some(c) => {
            match c {
                '"' => {
                    lexer.advance();
                    lexer.tokenize(Category::String);
                    Some(StateFunction(initial_state))
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
                    Some(StateFunction(initial_state))
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
    // Benchmarking
    extern crate test;
    use self::test::Bencher;

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
            Token{ lexeme: ":".to_string(), category: Category::AssignmentOperator },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"4032\"".to_string(), category: Category::String },
            Token{ lexeme: ",".to_string(), category: Category::Text },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "'single'".to_string(), category: Category::Text },
            Token{ lexeme: ":".to_string(), category: Category::AssignmentOperator },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "'quotes\\'',".to_string(), category: Category::Text },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"literals\"".to_string(), category: Category::String },
            Token{ lexeme: ":".to_string(), category: Category::AssignmentOperator },
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

    #[bench]
    fn bench_lex(b: &mut Bencher) {
        let data = include_str!("../../test_data/data.json");
        b.iter(|| lex(data));
    }
}
