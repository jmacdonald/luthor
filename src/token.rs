#[derive(PartialEq, Debug, Clone)]
pub enum Category {
    Whitespace,
    Identifier,
    Keyword,
    Brace,
    Bracket,
    Parenthesis,
    AssignmentOperator,
    Integer,
    Float,
    String,
    Boolean,
    Text,
    Comment,
    Method,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub lexeme: String,
    pub category: Category,
}
