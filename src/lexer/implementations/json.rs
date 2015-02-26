use lexer::new;
use lexer::Lexer;
use lexer::StateFunction;
use lexer::Token;
use lexer::Category;

fn initial_state(lexer: &mut Lexer) -> Option<StateFunction> {
    if lexer.token_position >= lexer.char_count {
        // There was uncategorized text before this; pass it along
        if lexer.token_position != lexer.token_start {
            lexer.tokens.push(Token{
                lexeme: lexer.data.slice_chars(lexer.token_start,
                    lexer.token_position).to_string(),
                category: Category::Text
            });
            lexer.token_start = lexer.token_position;
        }

        return None;
    }

    // TODO: Replace char_at with something UTF8 compatible.
    match lexer.data.char_at(lexer.token_position) {
        '{' => {
            // There was uncategorized text before this; pass it along
            if lexer.token_position != lexer.token_start {
                lexer.tokens.push(Token{
                    lexeme: lexer.data.slice_chars(lexer.token_start,
                        lexer.token_position).to_string(),
                    category: Category::Text
                });
                lexer.token_start = lexer.token_position;
            }

            lexer.tokens.push(Token{ lexeme: lexer.data.char_at(lexer.token_position).to_string(), category: Category::Brace });
            lexer.token_start += 1;
            lexer.token_position += 1;
            Some(StateFunction(initial_state))
        },
        '[' => {
            // There was uncategorized text before this; pass it along
            if lexer.token_position != lexer.token_start {
                lexer.tokens.push(Token{
                    lexeme: lexer.data.slice_chars(lexer.token_start,
                        lexer.token_position).to_string(),
                    category: Category::Text
                });
                lexer.token_start = lexer.token_position;
            }

            lexer.tokens.push(Token{ lexeme: lexer.data.char_at(lexer.token_position).to_string(), category: Category::Bracket });
            lexer.token_start += 1;
            lexer.token_position += 1;
            Some(StateFunction(initial_state))
        },
        ' ' | '\n' => {
            // There was uncategorized text before this; pass it along
            if lexer.token_position != lexer.token_start {
                lexer.tokens.push(Token{
                    lexeme: lexer.data.slice_chars(lexer.token_start,
                        lexer.token_position).to_string(),
                    category: Category::Text
                });
                lexer.token_start = lexer.token_position;
            }

            lexer.token_position += 1;
            Some(StateFunction(whitespace))
        },
        '"' => {
            // There was uncategorized text before this; pass it along
            if lexer.token_position != lexer.token_start {
                lexer.tokens.push(Token{
                    lexeme: lexer.data.slice_chars(lexer.token_start,
                        lexer.token_position).to_string(),
                    category: Category::Text
                });
                lexer.token_start = lexer.token_position;
            }

            lexer.token_position += 1;
            Some(StateFunction(inside_string))
        },
        ':' => {
            // There was uncategorized text before this; pass it along
            if lexer.token_position != lexer.token_start {
                lexer.tokens.push(Token{
                    lexeme: lexer.data.slice_chars(lexer.token_start,
                        lexer.token_position).to_string(),
                    category: Category::Text
                });
                lexer.token_start = lexer.token_position;
            }

            lexer.tokens.push(Token{ lexeme: lexer.data.char_at(lexer.token_position).to_string(), category: Category::AssignmentOperator });
            lexer.token_start += 1;
            lexer.token_position += 1;
            Some(StateFunction(initial_state))
        },
        '}' => {
            // There was uncategorized text before this; pass it along
            if lexer.token_position != lexer.token_start {
                lexer.tokens.push(Token{
                    lexeme: lexer.data.slice_chars(lexer.token_start,
                        lexer.token_position).to_string(),
                    category: Category::Text
                });
                lexer.token_start = lexer.token_position;
            }

            lexer.tokens.push(Token{ lexeme: lexer.data.char_at(lexer.token_position).to_string(), category: Category::Brace });
            None
        },
        ']' => {
            // There was uncategorized text before this; pass it along
            if lexer.token_position != lexer.token_start {
                lexer.tokens.push(Token{
                    lexeme: lexer.data.slice_chars(lexer.token_start,
                        lexer.token_position).to_string(),
                    category: Category::Text
                });
                lexer.token_start = lexer.token_position;
            }

            lexer.tokens.push(Token{ lexeme: lexer.data.char_at(lexer.token_position).to_string(), category: Category::Bracket });
            lexer.token_start += 1;
            lexer.token_position += 1;
            Some(StateFunction(initial_state))
        },
        _ => {
            if lexer.token_position == lexer.token_start {
                let remaining_data = lexer.data.slice_from(lexer.token_position);
                if remaining_data.starts_with("true") {
                    lexer.token_position += 4;
                    lexer.tokens.push(Token{
                        lexeme: lexer.data.slice_chars(lexer.token_start,
                            lexer.token_position).to_string(),
                        category: Category::Boolean
                    });
                    lexer.token_start = lexer.token_position;
                } else if remaining_data.starts_with("false") {
                    lexer.token_position += 5;
                    lexer.tokens.push(Token{
                        lexeme: lexer.data.slice_chars(lexer.token_start,
                            lexer.token_position).to_string(),
                        category: Category::Boolean
                    });
                    lexer.token_start = lexer.token_position;
                } else if remaining_data.starts_with("null") {
                    lexer.token_position += 4;
                    lexer.tokens.push(Token{
                        lexeme: lexer.data.slice_chars(lexer.token_start,
                            lexer.token_position).to_string(),
                        category: Category::Keyword
                    });
                    lexer.token_start = lexer.token_position;
                } else {
                    lexer.token_position += 1;
                }
            } else {
                lexer.token_position += 1;
            }
            Some(StateFunction(initial_state))
        }
    }
}

fn inside_string(lexer: &mut Lexer) -> Option<StateFunction> {
    match lexer.data.char_at(lexer.token_position) {
        '"' => {
            lexer.token_position += 1;
            lexer.tokens.push(Token{ lexeme: lexer.data.slice_chars(lexer.token_start, lexer.token_position).to_string(), category: Category::String });
            lexer.token_start = lexer.token_position;
            Some(StateFunction(initial_state))
        },
        '\\' => {
            lexer.token_position += 2;
            Some(StateFunction(inside_string))
        }
        _ => {
            lexer.token_position += 1;
            Some(StateFunction(inside_string))
        }
    }
}

fn whitespace(lexer: &mut Lexer) -> Option<StateFunction> {
    match lexer.data.char_at(lexer.token_position) {
        ' ' | '\n' => {
            lexer.token_position += 1;
            Some(StateFunction(whitespace))
        },
        _ => {
            lexer.tokens.push(Token{ lexeme: lexer.data.slice_chars(lexer.token_start, lexer.token_position).to_string(), category: Category::Whitespace });
            lexer.token_start = lexer.token_position;
            Some(StateFunction(initial_state))
        }
    }
}

fn lex(data: &str) -> Vec<Token> {
    let mut lexer = new(data);
    let mut state_function = StateFunction(initial_state);
    loop {
        let StateFunction(actual_function) = state_function;
        match actual_function(&mut lexer) {
            Some(f) => state_function = f,
            None => return lexer.tokens,
        }
    }
}

mod tests {
    use super::lex;
    use lexer::Token;
    use lexer::Category;
    use std::old_io::{File, Open, Read};

    #[test]
    fn it_works() {
        let data = File::open_mode(&Path::new("test_data/data.json"), Open, Read)
            .unwrap().read_to_string().unwrap();
        let tokens = lex(&data);
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
        ];

        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[index]);
        }
    }
}
