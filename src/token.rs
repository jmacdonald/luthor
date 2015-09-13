//! Token-related types.

/// The primary means of classifying a format or language's lexemes.
#[derive(PartialEq, Debug, Clone)]
pub enum Category {
    Whitespace,
    Identifier,
    Keyword,
    Brace,
    Bracket,
    Parenthesis,
    Operator,
    Integer,
    Float,
    String,
    Boolean,
    Text,
    Comment,
    Method,
    Call,
}

/// A lexeme and category pairing. Tokens are the final product of a lexer; 
/// their lexemes should join to produce the original data passed to the lexer.
#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub lexeme: String,
    pub category: Category,
}
