//! Luthor provides a collection of lexers for various formats and languages.
//! It also exposes types that aid in building lexers of your own.
pub mod lexers;
pub mod token;
mod tokenizer;

pub use tokenizer::{Tokenizer, StateFunction};
