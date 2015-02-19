enum Category {
    Identifier,
    AssignmentOperator,
    IntegerLiteral,
    StringLiteral,
}

pub struct Token {
    lexeme: String,
    category: Category,
}
