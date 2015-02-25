use lexer::new;
use lexer::Lexer;
use lexer::StateFunction;
use lexer::Token;
use lexer::Category;

fn initial_state(lexer: &mut Lexer) -> Option<StateFunction> {
    match lexer.data.char_at(lexer.token_position) {
        '{' => {
            lexer.tokens.push(Token{ lexeme: lexer.data.char_at(lexer.token_position).to_string(), category: Category::Brace });
            lexer.token_start += 1;
            lexer.token_position += 1;
            Some(StateFunction(inside_object))
        },
        _ => None
    }
}

fn inside_object(lexer: &mut Lexer) -> Option<StateFunction> {
    match lexer.data.char_at(lexer.token_position) {
        ' ' => {
            lexer.tokens.push(Token{ lexeme: lexer.data.char_at(lexer.token_position).to_string(), category: Category::Whitespace });
            lexer.token_start += 1;
            lexer.token_position += 1;
            Some(StateFunction(inside_object))
        },
        '"' => {
            lexer.token_position += 1;
            Some(StateFunction(inside_string))
        },
        ':' => {
            lexer.tokens.push(Token{ lexeme: lexer.data.char_at(lexer.token_position).to_string(), category: Category::AssignmentOperator });
            lexer.token_start += 1;
            lexer.token_position += 1;
            Some(StateFunction(inside_object))
        },
        '}' => {
            lexer.tokens.push(Token{ lexeme: lexer.data.char_at(lexer.token_position).to_string(), category: Category::Brace });
            None
        },
        _ => None
    }
}

fn inside_string(lexer: &mut Lexer) -> Option<StateFunction> {
    match lexer.data.char_at(lexer.token_position) {
        '"' => {
            lexer.token_position += 1;
            lexer.tokens.push(Token{ lexeme: lexer.data.slice_chars(lexer.token_start, lexer.token_position).to_string(), category: Category::StringLiteral });
            lexer.token_start = lexer.token_position;
            Some(StateFunction(inside_object))
        },
        _ => {
            lexer.token_position += 1;
            Some(StateFunction(inside_string))
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

    #[test]
    fn it_works() {
        let tokens = lex("{ \"villain\": \"luthor\" }");
        let expected_tokens = vec![
            Token{ lexeme: "{".to_string(), category: Category::Brace },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"villain\"".to_string(), category: Category::StringLiteral },
            Token{ lexeme: ":".to_string(), category: Category::AssignmentOperator },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "\"luthor\"".to_string(), category: Category::StringLiteral },
            Token{ lexeme: " ".to_string(), category: Category::Whitespace },
            Token{ lexeme: "}".to_string(), category: Category::Brace },
        ];

        assert_eq!(tokens, expected_tokens);
    }
}
