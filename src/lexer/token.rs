use crate::lexer::token_type::TokenType;

#[derive(Debug, Clone, PartialEq, Eq)]  // Add Clone her
pub struct Token {
    pub line: usize,
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub error_message: Option<String>
}

impl Token {
    pub fn null_token() -> Token {
        Token {
            line:0,
            token_type:TokenType::InitNull,
            start:0,
            length:0,
            error_message: None

        } 
    }
}

impl Token {
    pub fn new(line: usize, token_type: TokenType, start: usize, length: usize) -> Token {
       Token {
           line,
           token_type,
           start,
           length,
           error_message: None
     
       }
    }
    
    pub fn error_token(line: usize, start: usize, error_message: &str) -> Token {
        Token {
            line,
            token_type: TokenType::Error,
            start,
            length: error_message.len(),
            error_message: Some(error_message.to_string())
        }
    }
}