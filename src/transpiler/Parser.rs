use crate::lexer::lexer::Scanner;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use crate::transpiler::chunk::{Chunk, OpCode};
use crate::transpiler::chunk::OpCode::{OpAdd, OpDivide, OpMultiply, OpNegate, OpReturn, OpSubtract};
use crate::transpiler::Parser::Precedence::{Assignment, Unary};
use crate::transpiler::value::Types;

struct Parser<'a> {
    current: Token,
    prev: Token,
    had_error: bool,
    scanner: Scanner<'a>,
    panic_mode: bool,
    compiling_chunk: &'a mut Chunk
}

pub enum Precedence {
    None,
    Assignment,  // =
    Or,          // or
    And,         // and
    Equality,    // == !=
    Comparison,  // < > <= >=
    Term,        // + -
    Factor,      // *
    Unary,       // ! -
    Call,        // . ()
    Primary,
}
impl From<u8> for Precedence {
    // this allows us to let enumVal: OpCode = 0.into();
    // turn numbers to enum values because its all loosey goosey in c
    // and that loosey goosey ness is used a lot in c projects

    fn from(value: u8) -> Self {
        match value {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparison,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            10 => Precedence::Primary,
            _ => { panic!() }
        }
    }
}

impl Parser<'_> {
    pub(crate) fn end_compiler(&mut self) {
        self.emit_return();
    }
    pub fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current_with_message(message);
    }
    pub fn advance(&mut self) {
       self.prev = self.current.clone();

        loop {
            let token = self.scanner.scan_token();
            self.current = token;
            if self.current.token_type != TokenType::Error {break;}
            self.error_at_current();
        }

    }
    pub fn emit_byte(&mut self, byte: u8){
        self.compiling_chunk.write(byte, self.prev.line)
    }
    pub fn emit_bytes(&mut self, byte: u8, byte2: u8){
        self.emit_byte(byte);
        self.emit_byte(byte2);

    }
    fn emit_constant(&mut self, constant: f64) {
        self.compiling_chunk.write_constant(Types::Val(constant), self.prev.line);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpReturn as u8);
    }

    fn error_at_current(&mut self) {
        if self.panic_mode {return;}
        self.panic_mode = true;
        eprint!("[line {}] Error", self.current.line);
        if self.current.token_type == TokenType::Eof {
            eprint!(" at end");
        }else if self.current.token_type == TokenType::Error {

        }else {
            let source = self.scanner.source;
            eprint!(" at '{}'", &source[self.current.start..self.current.start + self.current.length]);
        }

        eprintln!(": {}", self.current.error_message.clone().unwrap());
        self.had_error = true;
    }
    fn error_at_current_with_message(&mut self, message: &str) {
        if self.panic_mode {return;}
        self.panic_mode = true;
        eprint!("[line {}] Error", self.current.line);
        if self.current.token_type == TokenType::Eof {
            eprint!(" at end");
        }else if self.current.token_type == TokenType::Error {

        }else {
            let source = self.scanner.source;
            eprint!(" at '{}'", &source[self.current.start..self.current.start + self.current.length]);
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }
    pub fn expression(&mut self) {
        self.parse_precedence(Assignment)
    }
    pub fn number(&mut self) {
        let st = self.prev.start;
        let len = self.prev.length;
        let slice = &self.scanner.source[st..=len];
        let num: f64 = slice.parse().unwrap();
        self.emit_constant(num);
    }
    pub fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }
    pub fn unary(&mut self) {
        let operator_type = self.prev.token_type;
        // self.expression();
        self.parse_precedence(Unary);
        match operator_type {
            TokenType::Minus => {
                self.emit_byte(OpNegate as u8);
            }
            TokenType::Bang => {}
            _ => {return;}
        }
    }
    pub fn binary(&mut self){
        let operator = self.prev.token_type;
        let rule: &ParseRule = ParseRule::get_rule(operator);
        let next_rule: u8 = rule.precedence+1;
        // this needs to be done because when parsing 2 * 3 + 4
        // for * 3+4 should not be parsed
        // it should only be parsed if the operator between 3 and 4 have a higher precedence than *
        // Since assignment is right-associative, we want to parse it as:
        // a = (b = (c = d))
        // To enable that, we would call parsePrecedence() with the same precedence as the current operator.
        self.parse_precedence(next_rule.into());
        match operator {
            TokenType::Minus => {self.emit_byte(OpSubtract as u8);}
            TokenType::Plus => {self.emit_byte(OpAdd as u8);}
            TokenType::Slash => {self.emit_byte(OpDivide as u8);}
            TokenType::Star => {self.emit_byte(OpMultiply as u8);}
            _ => {return;}
        }
    }
    pub fn parse_precedence(&mut self, precedence: Precedence){
        // parses any expression greater than equal to the precedence passed to it recursively

    }

}

pub fn compile(source: &mut String, chunk: &mut Chunk) -> bool {
    let mut scanner = Scanner::new(source);
    let mut parser: Parser = Parser {
        current: Token::null_token(),
        prev: Token::null_token(),
        had_error: false,
        scanner,
        panic_mode: false,
        compiling_chunk: chunk,
    };
    parser.advance();
    parser.expression();
    parser.consume(TokenType::Eof, "Expect end of expression.");
    parser.end_compiler();
    !parser.had_error
}

