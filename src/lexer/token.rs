#[derive(PartialEq, Debug)]
pub enum Category {
    Whitespace,
    Identifier,
    Brace,
    Parenthesis,
    AssignmentOperator,
    IntegerLiteral,
    StringLiteral,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub lexeme: String,
    pub category: Category,
}
