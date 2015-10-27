//! A simple lexer for HTML data with embedded Ruby. Breaks data into three
//! segments: HTML, erb tags, and Ruby. Defers to other lexers for the HTML
//! and Ruby segments.

use lexers;
use token::{Category, Token};
use tokenizer::{Tokenizer, StateFunction};

fn initial_state(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    // Check for (and tokenize) erb tags.
    if tokenizer.has_prefix("<%=") {
        tokenizer.tokenize_next(3, Category::Keyword);
        return Some(StateFunction(ruby))
    } else if tokenizer.has_prefix("<%") {
        tokenizer.tokenize_next(2, Category::Keyword);
        return Some(StateFunction(ruby))
    }

    match tokenizer.current_char() {
        Some(_) => {
            tokenizer.advance();
            Some(StateFunction(initial_state))
        },
        None => {
            tokenizer.tokenize(Category::Text);
            None
        }
    }
}

fn ruby(tokenizer: &mut Tokenizer) -> Option<StateFunction> {
    // Check for (and tokenize) erb exit tag.
    if tokenizer.has_prefix("%>") {
        tokenizer.tokenize(Category::String);
        tokenizer.tokenize_next(2, Category::Keyword);
        return Some(StateFunction(initial_state))
    }

    match tokenizer.current_char() {
        Some(_) => {
            tokenizer.advance();
            Some(StateFunction(ruby))
        },
        None => {
            tokenizer.tokenize(Category::String);
            None
        }
    }
}

pub fn lex(data: &str) -> Vec<Token> {
    // Lex the data into three categories; one for html segments.
    // another for erb tags, and yet another for Ruby segments.
    let mut tokenizer = Tokenizer::new(data);
    let mut state_function = StateFunction(initial_state);
    loop {
        let StateFunction(actual_function) = state_function;
        match actual_function(&mut tokenizer) {
            Some(f) => state_function = f,
            None => break,
        }
    }

    // Defer to other lexers for HTML and Ruby segments, and combine the sets.
    tokenizer.tokens().iter().fold(
        Vec::new(),
        |mut tokens, token| {
            match token.category {
                Category::Keyword => tokens.push(token.clone()),
                Category::String => tokens.extend(lexers::ruby::lex(&token.lexeme).into_iter()),
                Category::Text => tokens.extend(lexers::xml::lex(&token.lexeme).into_iter()),
                _ => (),
            };

            tokens
        }
    )
}

#[cfg(test)]
mod tests {
    use token::{Category, Token};

    #[test]
    fn it_works() {
        let data = include_str!("../../test_data/html_erb.html.erb");
        let tokens = super::lex(data);

        let expected_tokens = vec![
            Token{ lexeme: "<".to_string(), category: Category::Text },
            Token{ lexeme: "html".to_string(), category: Category::Identifier },
            Token{ lexeme: ">".to_string(), category: Category::Text },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "<%".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "class".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "Ruby".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "%>".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "<%=".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "class".to_string(), category: Category::Keyword },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "Ruby".to_string(), category: Category::Identifier },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "%>".to_string(), category: Category::Keyword },
            Token{ lexeme: "\n".to_string(), category: Category::Whitespace },
            Token{ lexeme: "</".to_string(), category: Category::Text },
            Token{ lexeme: "html".to_string(), category: Category::Identifier },
            Token{ lexeme: ">".to_string(), category: Category::Text },
        ];

        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[index]);
        }
    }
}
