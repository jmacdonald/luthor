//! A lexer for the Ruby programming language.

use token::{Category, Token};
use tokenizer::{Tokenizer, StateFunction};

fn initial_state(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    if tokenizer.starts_with_lexeme("class") {
        tokenizer.tokenize_next(5, Category::Keyword);
        tokenizer.states.push(StateFunction(identifier));
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("module") {
        tokenizer.tokenize_next(6, Category::Keyword);
        tokenizer.states.push(StateFunction(identifier));
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("include") {
        tokenizer.tokenize_next(7, Category::Keyword);
        tokenizer.states.push(StateFunction(identifier));
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("extend") {
        tokenizer.tokenize_next(6, Category::Keyword);
        tokenizer.states.push(StateFunction(identifier));
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("def") {
        tokenizer.tokenize_next(3, Category::Keyword);
        tokenizer.states.push(StateFunction(method));
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("do") {
        tokenizer.tokenize_next(2, Category::Keyword);
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("if") {
        tokenizer.tokenize_next(2, Category::Keyword);
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("unless") {
        tokenizer.tokenize_next(6, Category::Keyword);
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("elsif") {
        tokenizer.tokenize_next(5, Category::Keyword);
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("else") {
        tokenizer.tokenize_next(4, Category::Keyword);
        return Some(StateFunction(whitespace))
    } else if tokenizer.starts_with_lexeme("end") {
        tokenizer.tokenize_next(3, Category::Keyword);
        return Some(StateFunction(initial_state))
    } else if tokenizer.starts_with_lexeme("true") {
        tokenizer.tokenize_next(4, Category::Boolean);
        return Some(StateFunction(initial_state))
    } else if tokenizer.starts_with_lexeme("false") {
        tokenizer.tokenize_next(5, Category::Boolean);
        return Some(StateFunction(initial_state))
    } else if tokenizer.starts_with_lexeme("nil") {
        tokenizer.tokenize_next(3, Category::Literal);
        return Some(StateFunction(initial_state))
    } else if tokenizer.starts_with_lexeme("private") {
        tokenizer.tokenize_next(7, Category::Keyword);
        return Some(StateFunction(initial_state))
    } else if tokenizer.starts_with_lexeme("begin") {
        tokenizer.tokenize_next(5, Category::Keyword);
        return Some(StateFunction(initial_state))
    } else if tokenizer.starts_with_lexeme("rescue") {
        tokenizer.tokenize_next(6, Category::Keyword);
        return Some(StateFunction(initial_state))
    } else if tokenizer.starts_with_lexeme("raise") {
        tokenizer.tokenize_next(5, Category::Keyword);
        return Some(StateFunction(initial_state))
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
        Some('#') => {
            tokenizer.tokenize(Category::Text);
            tokenizer.advance();
            tokenizer.states.push(StateFunction(initial_state));
            Some(StateFunction(comment))
        },
        Some('|') => {
            tokenizer.tokenize_next(1, Category::Text);
            tokenizer.states.push(StateFunction(argument));
            Some(StateFunction(whitespace))
        },
        Some('.') => {
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
        Some('@') => {
            tokenizer.tokenize(Category::Text);
            Some(StateFunction(identifier))
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
        Some(':') => {
            if tokenizer.has_prefix(": ") {
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

fn method(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    match tokenizer.current_char() {
        Some(c) => {
            match c {
                ' ' | '\n' => {
                    tokenizer.tokenize(Category::Method);
                    Some(StateFunction(initial_state))
                },
                '(' => {
                    tokenizer.tokenize(Category::Method);
                    tokenizer.tokenize_next(1, Category::Text);
                    Some(StateFunction(argument))
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

/// Lexes a Ruby document.
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
        let data = include_str!("../../test_data/ruby.rb");
        let tokens = lex(data);
        let expected_tokens = vec![
            Token{ lexeme: "module".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "RubyModule".to_string(), category: Category::Identifier },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "end".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "class".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "Ruby".to_string(), category: Category::Identifier },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "include".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "RubyModule".to_string(), category: Category::Identifier },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "extend".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "RubyModule".to_string(), category: Category::Identifier },
            Token{ lexeme: "\n\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "private".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "def".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "method".to_string(), category: Category::Method },
            Token{ lexeme: "(".to_string(), category: Category::Text },
            Token{ lexeme: "argument".to_string(), category: Category::Identifier },
            Token{ lexeme: ")".to_string(), category: Category::Text },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "begin".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "unless".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "if".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "true".to_string(), category: Category::Boolean },
            Token{ lexeme: "\n        ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"true\"".to_string(), category: Category::String },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "elsif".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "false".to_string(), category: Category::Boolean },
            Token{ lexeme: "\n        ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"false\"".to_string(), category: Category::String },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "else".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n        ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "nil".to_string(), category: Category::Literal },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "end".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "rescue".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "raise".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "end".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "# comment".to_string(), category: Category::Comment },
            Token{ lexeme: "\n    ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "[".to_string(), category: Category::Text },
            Token{ lexeme: "\"ruby\"".to_string(), category: Category::String },
            Token{ lexeme: "]".to_string(), category: Category::Text },
            Token{ lexeme: ".".to_string(), category: Category::Text },
            Token{ lexeme: "each".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "do".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "|".to_string(), category: Category::Text },
            Token{ lexeme: "string".to_string(), category: Category::Identifier },
            Token{ lexeme: "|".to_string(), category: Category::Text },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "variable".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "=".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "'string'".to_string(), category: Category::String },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "another_variable".to_string(), category: Category::Identifier },
            Token{ lexeme: "=".to_string(), category: Category::Text },
            Token{ lexeme: "1".to_string(), category: Category::Integer },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "@instance_variable".to_string(), category: Category::Identifier },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "method_call".to_string(), category: Category::Call },
            Token{ lexeme: "(".to_string(), category: Category::Text },
            Token{ lexeme: "argument".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "=".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "false".to_string(), category: Category::Boolean },
            Token{ lexeme: ",".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "another_argument".to_string(), category: Category::Identifier },
            Token{ lexeme: ")".to_string(), category: Category::Text },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "another_method_call".to_string(), category: Category::Text },
            Token{ lexeme: "\n      ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "hash".to_string(), category: Category::Identifier },
            Token{ lexeme: "[".to_string(), category: Category::Text },
            Token{ lexeme: ":symbol_1234?".to_string(), category: Category::Literal },
            Token{ lexeme: "]".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "=".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "{".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "key".to_string(), category: Category::Literal },
            Token{ lexeme: ":".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "value".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "}".to_string(), category: Category::Text },
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
