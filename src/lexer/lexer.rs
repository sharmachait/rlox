use crate::lexer::token::Token;
use crate::lexer::token_type::*;

struct Scanner<'a> {
    source: & 'a String,
    start: usize, //marks the beginning of the current lexeme being scanned
    current: usize, //points to the current character being looked at
    line: u32
}

// At any point in time, the compiler needs only one or two tokens
// we don’t need to keep them all around at the same time

impl<'a> Scanner<'a>{
    fn new(source: &'a String) -> Self{
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;

        if self.is_at_end() {
            let token_length  = self.current - self.start;
            return Token::new(self.line, TokenType::Eof, self.start, token_length);
        }
        let token_length  =self.current - self.start;
        Token::error_token(self.line, self.start, "Unexpected character.")
    }

    fn is_at_end(&self) -> bool {
        // self.current == self.source.len() -1 // maybe
        self.current == self.source.len()
    }
}

pub fn lex(source: &mut String) {
    let mut scanner = Scanner::new(source);

    let mut line: i32 = -1;
    loop {
        let token = scanner.scan_token();
        if line == -1 || token.line != line as u32 {
            print!("{:4} ", token.line);
            line = token.line as i32;
        } else {
            print!("   | ");
        }

        if let TokenType::Error = token.token_type {
            println!("{:2} '{}'",
                     &token.token_type,
                     token.error_message.unwrap()
            );
            scanner.current = scanner.current+1;
        }else{
            println!("{:2} '{}'",
                     &token.token_type,
                     &source[token.start..token.start + token.length]
            );
        }

        if let TokenType::Eof = token.token_type{break;}
    }
}