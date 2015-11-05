//! A lexer for the Ruby programming language.

use token::{Category, Token};
use tokenizer::{Tokenizer, StateFunction};

fn initial_state(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    if tokenizer.starts_with_lexeme("function") {
        tokenizer.tokenize_next(8, Category::Keyword);
        tokenizer.states.push(StateFunction(function));
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("if") {
        tokenizer.tokenize_next(2, Category::Keyword);
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("else") {
        tokenizer.tokenize_next(4, Category::Keyword);
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("return") {
        tokenizer.tokenize_next(6, Category::Keyword);
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("true") {
        tokenizer.tokenize_next(4, Category::Boolean);
        return Some(StateFunction(initial_state))
    } else if tokenizer.starts_with_lexeme("false") {
        tokenizer.tokenize_next(5, Category::Boolean);
        return Some(StateFunction(initial_state))
    } else if tokenizer.starts_with_lexeme("var") {
        tokenizer.tokenize_next(3, Category::Keyword);
        tokenizer.consume_whitespace();
        return Some(StateFunction(identifier))
    } else if tokenizer.has_prefix("//") {
        tokenizer.tokenize(Category::Text);
        return Some(StateFunction(comment))
    } else if tokenizer.has_prefix("/*") {
        tokenizer.tokenize(Category::Text);
        tokenizer.advance();
        tokenizer.advance();
        return Some(StateFunction(multi_line_comment))
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
        Some('.') => {
            tokenizer.tokenize_next(1, Category::Text);
            Some(StateFunction(initial_state))
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
        Some(':') => {
            if tokenizer.starts_with_lexeme(":") {
                tokenizer.tokenize(Category::Literal);
                tokenizer.tokenize_next(1, Category::Text);
                tokenizer.consume_whitespace();
                Some(StateFunction(initial_state))
            } else {
                tokenizer.advance();
                Some(StateFunction(symbol))
            }
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
                '|' | '(' | ')' | '-' | ';' | '{' | '}' => {
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

fn multi_line_comment(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    if tokenizer.has_prefix("*/") {
        tokenizer.advance();
        tokenizer.advance();
        tokenizer.tokenize(Category::Comment);
        return Some(StateFunction(initial_state))
    }

    match tokenizer.current_char() {
        Some(_) => {
            tokenizer.advance();
            Some(StateFunction(multi_line_comment))
        },
        None => {
            tokenizer.tokenize(Category::Comment);
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

fn symbol(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            if c.is_alphanumeric() || c == '_' || c == '?' {
                tokenizer.advance();
                Some(StateFunction(symbol))
            } else {
                tokenizer.tokenize(Category::Literal);
                Some(StateFunction(initial_state))
            }
        }

        None => {
            tokenizer.tokenize(Category::Literal);
            None
        }
    }
}

/// Lexes a JavaScript document.
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
        let data = include_str!("../../test_data/data.js");
        let tokens = lex(data);
        let expected_tokens = vec![
            Token{ lexeme: "var".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "data".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "=".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"string\"".to_string(), category: Category::String },
            Token{ lexeme: ";".to_string(), category: Category::Text },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "var".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "data_2".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "=".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "'string'".to_string(), category: Category::String },
            Token{ lexeme: ";".to_string(), category: Category::Text },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "// comment".to_string(), category: Category::Comment },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "/*\n multi-line comment\n*/".to_string(), category: Category::Comment },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "function".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "myFunction".to_string(), category: Category::Function },
            Token{ lexeme: "(".to_string(), category: Category::Text },
            Token{ lexeme: "arg".to_string(), category: Category::Identifier },
            Token{ lexeme: ")".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "{".to_string(), category: Category::Text },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "if".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "true".to_string(), category: Category::Boolean },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "{}".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "else".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "{".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "return".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "false".to_string(), category: Category::Boolean },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "}".to_string(), category: Category::Text },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "}".to_string(), category: Category::Text },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
        ];

        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[index]);
        }
    }

    #[test]
    fn it_identifies_integers_and_operators() {
        let data = "123 + 456";
        let tokens = lex(data);
        let expected_tokens = vec![
            Token{ lexeme: "123".to_string(), category: Category::Integer },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "+".to_string(), category: Category::Operator },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "456".to_string(), category: Category::Integer },
        ];

        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[index]);
        }
    }
}
