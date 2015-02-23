#[derive(PartialEq, Show)]
pub enum Category {
    Whitespace,
    Identifier,
    Brace,
    Parenthesis,
    AssignmentOperator,
    IntegerLiteral,
    StringLiteral,
}

pub struct Token {
    pub lexeme: String,
    pub category: Category,
}
