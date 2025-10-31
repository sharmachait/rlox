use crate::lexer::token_type::TokenType;

pub struct Token {
    pub line: u32,
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub error_message: Option<String>
}

impl Token {
    pub fn new(line: u32, token_type: TokenType, start: usize, length: usize) -> Token {
       Token {
           line,
           token_type,
           start,
           length,
           error_message: None
       }
    }
    pub fn error_token(line: u32, start: usize, error_message: &str) -> Token {
        Token {
            line,
            token_type: TokenType::Error,
            start,
            length: error_message.len(),
            error_message: Some(error_message.to_string())
        }
    }
}