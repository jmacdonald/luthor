//! A simple lexer that will produce text and whitespace categorized tokens,
//! suitable as a fallback in situations where a format/language-specific
//! equivalent is unavailable.

use tokenizer::new;
use tokenizer::Tokenizer;
use tokenizer::StateFunction;
use token::Token;
use token::Category;

fn initial_state(lexer: &mut Tokenizer) -> Option<StateFunction> {
    match lexer.current_char() {
        Some(c) => {
            match c {
                ' ' | '\n' => {
                    lexer.tokenize(Category::Text);
                    lexer.advance();
                    return Some(StateFunction(whitespace));
                },
                _ => lexer.advance(),
            }

            Some(StateFunction(initial_state))
        }

        None => {
            lexer.tokenize(Category::Text);
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

/// Lexes any UTF-8 document.
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
        let data = include_str!("../../test_data/data.txt");
        let tokens = lex(data);
        let expected_tokens = vec![
            Token{ lexeme: "This".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "is".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "a".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "test.".to_string(), category: Category::Text },
            Token{ lexeme: "\n  ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "Luthor".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "text".to_string(), category: Category::Text },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "lexing.".to_string(), category: Category::Text },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
        ];

        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[index]);
        }
    }
}
