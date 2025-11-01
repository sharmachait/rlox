use crate::lexer::token::Token;
use crate::lexer::token_type::*;

pub struct Scanner<'a> {
    source: & 'a String,
    start: usize, //marks the beginning of the current lexeme being scanned
    current: usize, //points to the current character being looked at
    line: u32
}

// At any point in time, the compiler needs only one or two tokens
// we don’t need to keep them all around at the same time

impl<'a> Scanner<'a>{
    pub fn new(source: &'a String) -> Self{
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            let token_length  = 0;
            return Token::new(self.line, TokenType::Eof, self.start, token_length);
        }

        let c: char = self.advance();

        if is_alpha(c) {
            return self.identifier();
        }

        if c.is_numeric() {
            return self.number();
        }

        match c {
            '(' =>  Token::new(self.line, TokenType::LeftParen, self.start, 1),
            ')' =>  Token::new(self.line, TokenType::RightParen, self.start, 1),
            '{' =>  Token::new(self.line, TokenType::LeftBrace, self.start, 1),
            '}' =>  Token::new(self.line, TokenType::RightBrace, self.start, 1),
            ';' =>  Token::new(self.line, TokenType::Semicolon, self.start, 1),
            ',' =>  Token::new(self.line, TokenType::Comma, self.start, 1),
            '.' =>  Token::new(self.line, TokenType::Dot, self.start, 1),
            '-' =>  Token::new(self.line, TokenType::Minus, self.start, 1),
            '+' =>  Token::new(self.line, TokenType::Plus, self.start, 1),
            '/' =>  Token::new(self.line, TokenType::Slash, self.start, 1),
            '*' =>  Token::new(self.line, TokenType::Star, self.start, 1),
            '!' =>  {
                if self.match_next_token_and_advance('=') {
                    Token::new(self.line, TokenType::BangEqual, self.start, 2)
                } else {
                    Token::new(self.line, TokenType::Bang, self.start, 1)
                }
            },
            '=' =>  {
                if self.match_next_token_and_advance('=') {
                    Token::new(self.line, TokenType::EqualEqual, self.start, 2)
                } else {
                    Token::new(self.line, TokenType::Equal, self.start, 1)
                }
            },
            '<' =>  {
                if self.match_next_token_and_advance('=') {
                    Token::new(self.line, TokenType::LessEqual, self.start, 2)
                } else {
                    Token::new(self.line, TokenType::Less, self.start, 1)
                }
            }
            '>' =>  {
                if self.match_next_token_and_advance('=') {
                    Token::new(self.line, TokenType::GreaterEqual, self.start, 2)
                } else {
                    Token::new(self.line, TokenType::Greater, self.start, 1)
                }
            },
            '"' => self.string(),
            _ =>  Token::error_token(self.line, self.start, "Unexpected character.")
        }
    }

    fn is_at_end(&self) -> bool {
        // self.current == self.source.len() -1 // maybe
        self.current == self.source.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current..].chars().next().unwrap();
        self.current += ch.len_utf8();
        ch
    }

    fn match_next_token_and_advance(&mut self, expected_token: char) -> bool {
        if self.is_at_end() { return false; }

        let next_char = match self.source[self.current..].chars().next() {
            Some(ch) => ch,
            None => return false,
        };

        if next_char != expected_token { return false }

        self.current += next_char.len_utf8();
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            let next_char = self.peek();
            match next_char {
                ' ' => { self.advance(); },
                '\r' => { self.advance(); },
                '\t' => { self.advance(); },
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                '/' => {
                    if self.peek_next() == '/' {
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                },
                _ => return
            };
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.source[self.current..].chars().next().unwrap()
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {return '\0'; }
        if self.current+1 >= self.source.len() { return '\0';}
        self.source[self.current..].chars().nth(1).unwrap()
    }

    fn string(&mut self) -> Token {
        while !self.is_at_end() && self.peek()!='"' {
            if self.peek() == '\n' {self.line+=1;}
            self.advance();
        }

        if self.is_at_end() { return Token::error_token(self.line, self.start, "Unexpected character."); }
        self.advance();
        let len = self.current - self.start;
        Token::new(self.line, TokenType::String, self.start, len)
    }

    fn number(&mut self) -> Token {
        while self.peek().is_numeric() {self.advance();}

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();
            while self.peek().is_numeric() {self.advance();}
        }

        let len = self.current - self.start;
        Token::new(self.line, TokenType::Number, self.start, len)
    }

    fn identifier(&mut self) -> Token {
        while !self.is_at_end() && (is_alpha(self.peek()) || self.peek().is_numeric()) {self.advance();}

        let len = self.current - self.start;
        Token::new(self.line, self.identifier_type(), self.start, len)
    }

    fn identifier_type(&self) -> TokenType {
        let c = self.source[self.start..].chars().next().unwrap();
        match c {
            'a' => self.check_keyword(1,2,"nd",TokenType::And),
            'c' => self.check_keyword(1,4,"lass",TokenType::Class),
            'e' => self.check_keyword(1,3,"lse",TokenType::Else),
            'f' => {
                if self.current - self.start <= 1 {
                    return TokenType::Identifier
                }
                let c = self.source[self.start+1..].chars().next().unwrap();
                match c {
                    'a' => self.check_keyword(2,3,"lse",TokenType::False),
                    'o' => self.check_keyword(2,1,"r",TokenType::For),
                    'u' => self.check_keyword(2,1,"n",TokenType::Fun),
                    _ => TokenType::Identifier
                }
            },
            'i' => self.check_keyword(1,1,"f",TokenType::If),
            'n' => self.check_keyword(1,2,"il",TokenType::Nil),
            'o' => self.check_keyword(1,1,"r",TokenType::Or),
            'p' => self.check_keyword(1,4,"rint",TokenType::Print),
            'r' => self.check_keyword(1,5,"eturn",TokenType::Return),
            's' => self.check_keyword(1,4,"uper",TokenType::Super),
            't' => {
                if self.current - self.start <= 1 {
                    return TokenType::Identifier
                }
                let c = self.source[self.start+1..].chars().next().unwrap();
                match c {
                    'h' => self.check_keyword(2,2,"is",TokenType::This),
                    'r' => self.check_keyword(2,2,"ue",TokenType::True),
                    _ => TokenType::Identifier
                }
            },
            'v' => self.check_keyword(1,2,"ar",TokenType::Var),
            'w' => self.check_keyword(1,4,"hile",TokenType::While),
            _ => TokenType::Identifier
        }
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, token_type: TokenType) -> TokenType {
        let parsed_length = self.current - self.start;
        let slice = &self.source[self.start + start .. self.current];
        if parsed_length == start + length && slice == rest {
            return token_type;
        }
        TokenType::Identifier
    }
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
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
            scanner.advance();
        }else{
            println!("{:2} '{}'",
                     &token.token_type,
                     &source[token.start..token.start + token.length]
            );
        }

        if let TokenType::Eof = token.token_type{break;}
    }
}