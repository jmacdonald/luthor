use super::super::Lexer;
use super::super::StateFunction;
use super::super::token::Token;
use super::super::token::Category;

pub fn initial_state(lexer: &mut Lexer) -> Option<StateFunction> {
    match lexer.data.char_at(lexer.token_position) {
        '{' => {
            lexer.tokens.push(Token{ lexeme: lexer.data.char_at(lexer.token_position).to_string(), category: Category::Brace });
            Some(StateFunction{ f: initial_state })
        },
        _ => None
    }
}
