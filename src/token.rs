//! Token-related types.

use std::ops::{Deref, DerefMut};

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
    Function,
    Method,
    Call,
    Literal,
    Key,
}

/// A lexeme and category pairing. Tokens are the final product of a lexer;
/// their lexemes should join to produce the original data passed to the lexer.
#[derive(PartialEq, Debug, Clone)]
pub struct Token<'a> {
    pub lexeme: &'a str,
    pub category: Category,
}

/// Holds text data and a set of tokens (categorized slices) referencing it.
///
/// # Examples
///
/// ```
/// use luthor::token::{Category, Token, TokenSet};
///
/// let data = String::from("luthor");
/// let mut token_set = TokenSet::new(data);
/// token_set.tokens.push(
///     Token{
///         lexeme: &token_set.data[0..6],
///         category: Category::Text
///     }
/// );
///
/// assert_eq!(token_set.first().unwrap().lexeme, "luthor");
/// ```
pub struct TokenSet<'a> {
    pub data: String,
    pub tokens: Vec<Token<'a>>
}

impl<'a> Deref for TokenSet<'a> {
    type Target = Vec<Token<'a>>;

    fn deref(&self) -> &Vec<Token<'a>> {
        &self.tokens
    }
}

impl<'a> DerefMut for TokenSet<'a> {
    fn deref_mut(&mut self) -> &mut Vec<Token<'a>> {
        &mut self.tokens
    }
}

impl<'a> TokenSet<'a> {
    pub fn new(data: String) -> TokenSet<'a> {
        TokenSet{
            data: data,
            tokens: Vec::new()
        }
    }
}
