use super::super::Lexer;
use super::super::StateFunction;
use super::super::token::Token;
use super::super::token::Category;

pub fn initial_state(potential_token: &str, tokens: &mut Vec<Token>) -> Option<StateFunction> {
    match potential_token {
        "{" => {
            tokens.push(Token{ lexeme: potential_token.to_string(), category: Category::Brace });
            Some(StateFunction{ f: initial_state })
        },
        _ => None
    }
}
