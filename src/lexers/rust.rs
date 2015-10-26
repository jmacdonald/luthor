//! A lexer for the Ruby programming language.

use token::{Category, Token};
use tokenizer::{Tokenizer, StateFunction};

fn initial_state(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    for keyword in vec!["pub", "let", "mut", "match", "loop"] {
        if tokenizer.starts_with_lexeme(keyword) {
            tokenizer.tokenize_next(keyword.chars().count(), Category::Keyword);

            return Some(StateFunction(initial_state))
        }
    }

    if tokenizer.starts_with_lexeme("use") {
        tokenizer.tokenize_next(3, Category::Keyword);
        tokenizer.states.push(StateFunction(identifier));

        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("mod") {
        tokenizer.tokenize_next(3, Category::Keyword);
        tokenizer.states.push(StateFunction(identifier));

        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("extern") {
        tokenizer.tokenize_next(6, Category::Keyword);
        tokenizer.states.push(StateFunction(initial_state));

        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("crate") {
        tokenizer.tokenize_next(5, Category::Keyword);
        tokenizer.states.push(StateFunction(identifier));

        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("for") {
        tokenizer.tokenize_next(3, Category::Keyword);
        tokenizer.states.push(StateFunction(identifier));

        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("in") {
        tokenizer.tokenize_next(2, Category::Keyword);
        tokenizer.states.push(StateFunction(identifier));

        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("fn") {
        tokenizer.tokenize_next(2, Category::Keyword);
        tokenizer.states.push(StateFunction(function));

        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme(":") {
        tokenizer.advance();
        tokenizer.tokenize(Category::Key);
        tokenizer.states.push(StateFunction(identifier));

        return Some(StateFunction(whitespace))
    } else if tokenizer.has_prefix("::") {
        tokenizer.tokenize(Category::Identifier);
        tokenizer.tokenize_next(2, Category::Text);

        return Some(StateFunction(initial_state))
    } else if tokenizer.starts_with_lexeme("//") {
        return Some(StateFunction(comment))
    }

    match tokenizer.current_char() {
        Some('"') => {
            tokenizer.tokenize(Category::Text);
            tokenizer.advance();
            Some(StateFunction(inside_string))
        },
        Some('\'') => {
            tokenizer.tokenize(Category::Text);
            tokenizer.advance();
            Some(StateFunction(inside_single_quote_string))
        },
        Some('|') => {
            tokenizer.tokenize_next(1, Category::Text);
            tokenizer.states.push(StateFunction(argument));
            Some(StateFunction(whitespace))
        },
        Some('.') => {
            tokenizer.tokenize(Category::Identifier);
            tokenizer.tokenize_next(1, Category::Text);
            Some(StateFunction(initial_state))
        },
        Some('(') => {
            tokenizer.tokenize(Category::Call);
            tokenizer.tokenize_next(1, Category::Text);
            Some(StateFunction(argument))
        },
        Some('+') => {
            tokenizer.tokenize_next(1, Category::Operator);
            Some(StateFunction(initial_state))
        },
        Some(' ') | Some('\n') => {
            match tokenizer.next_non_whitespace_char() {
                Some('=') => {
                    tokenizer.tokenize(Category::Identifier);
                    tokenizer.consume_whitespace();
                    tokenizer.tokenize_next(1, Category::Text);
                },
                _ => tokenizer.consume_whitespace(),
            }
            Some(StateFunction(initial_state))
        },
        Some('=') => {
            tokenizer.tokenize(Category::Identifier);
            tokenizer.tokenize_next(1, Category::Text);
            Some(StateFunction(initial_state))
        },
        Some('[') => {
            tokenizer.tokenize(Category::Identifier);
            tokenizer.tokenize_next(1, Category::Text);
            Some(StateFunction(initial_state))
        },
        Some(']') => {
            tokenizer.tokenize_next(1, Category::Text);
            Some(StateFunction(initial_state))
        },
        Some(c) => {
            tokenizer.advance();

            if c.is_numeric() {
                Some(StateFunction(integer))
            } else {
                Some(StateFunction(initial_state))
            }
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
                ':' => {
                    tokenizer.advance();
                    tokenizer.tokenize(Category::Identifier);
                    Some(StateFunction(initial_state))
                },
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
                    match tokenizer.states.pop() {
                        Some(state) => Some(state),
                        None => Some(StateFunction(initial_state)),
                    }
                }
            }
        }

        None => {
            tokenizer.tokenize(Category::Whitespace);
            None
        }
    }
}

fn argument(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    if tokenizer.starts_with_lexeme("true") {
        tokenizer.tokenize_next(4, Category::Boolean);
        return Some(StateFunction(argument))
    } else if tokenizer.starts_with_lexeme("false") {
        tokenizer.tokenize_next(5, Category::Boolean);
        return Some(StateFunction(argument))
    }

    match tokenizer.current_char() {
        Some(c) => {
            match c {
                ' ' | '\n' => {
                    tokenizer.tokenize(Category::Identifier);
                    tokenizer.states.push(StateFunction(argument));
                    Some(StateFunction(whitespace))
                },
                '|' | ')' => {
                    tokenizer.tokenize(Category::Identifier);
                    tokenizer.tokenize_next(1, Category::Text);
                    Some(StateFunction(initial_state))
                },
                '=' | ',' => {
                    tokenizer.tokenize(Category::Identifier);
                    tokenizer.tokenize_next(1, Category::Text);
                    Some(StateFunction(argument))
                },
                _ => {
                    tokenizer.advance();
                    Some(StateFunction(argument))
                }
            }
        }

        None => {
            tokenizer.tokenize(Category::Identifier);
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
                '|' | '(' | ')' | '-' | ';' => {
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
            tokenizer.tokenize(Category::Identifier);
            None
        }
    }
}

fn function(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            match c {
                ' ' | '\n' => {
                    tokenizer.tokenize(Category::Function);
                    Some(StateFunction(initial_state))
                },
                '(' => {
                    tokenizer.tokenize(Category::Function);
                    tokenizer.tokenize_next(1, Category::Text);
                    Some(StateFunction(argument))
                },
                _ => {
                    tokenizer.advance();
                    Some(StateFunction(function))
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

fn integer(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            if c.is_numeric() {
                tokenizer.advance();
                Some(StateFunction(integer))
            } else {
                tokenizer.tokenize(Category::Integer);
                Some(StateFunction(initial_state))
            }
        }

        None => {
            tokenizer.tokenize(Category::Integer);
            None
        }
    }
}

pub fn lex(data: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(data);
    let mut state_function = StateFunction(initial_state);
    loop {
        let StateFunction(actual_function) = state_function;
        match actual_function(&mut tokenizer) {
            Some(f) => state_function = f,
            None => {
                match tokenizer.states.pop() {
                    Some(f) => state_function = f,
                    None => return tokenizer.tokens(),
                }
            }
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
        let data = include_str!("../../test_data/rust.rs");
        let tokens = lex(data);
        let expected_tokens = vec![
            Token{ lexeme: "extern".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "crate".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "luthor".to_string(), category: Category::Identifier },
            Token{ lexeme: ";".to_string(), category: Category::Text },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "use".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "luthor".to_string(), category: Category::Identifier },
            Token{ lexeme: ";".to_string(), category: Category::Text },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "pub".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "fn".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "main".to_string(), category: Category::Function },
            Token{ lexeme: "(".to_string(), category: Category::Text },
            Token{ lexeme: ")".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "{".to_string(), category: Category::Text },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "let".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "mut".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "variable".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "=".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"string\"".to_string(), category: Category::String },
            Token{ lexeme: ";".to_string(), category: Category::Text },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "'loop_name:".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "for".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "value".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "in".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "collection".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "{}".to_string(), category: Category::Text },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "}".to_string(), category: Category::Text },
        ];

        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[index]);
        }
    }
}
