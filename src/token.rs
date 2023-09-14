use std::any::Any;
use crate::TokenType;

#[derive(Debug)]
pub struct Token{
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Box<dyn Any>,
    pub line: usize
}
