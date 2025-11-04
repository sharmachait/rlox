use crate::lexer::lexer::Scanner;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use crate::lexer::token_type::TokenType::Eof;
use crate::transpiler::chunk::{Chunk, OpCode};
use crate::transpiler::chunk::OpCode::{OpAdd, OpDivide, OpEqual, OpFalse, OpGreater, OpLess, OpMultiply, OpNegate, OpNil, OpNot, OpReturn, OpSubtract, OpTrue};
use crate::transpiler::debug::disassemble;
use crate::transpiler::parser::Precedence::{Assignment, Unary};
use crate::transpiler::value::{Obj, Value};

struct Parser<'a> {
    current: Token,
    prev: Token,
    had_error: bool,
    scanner: Scanner<'a>,
    panic_mode: bool,
    compiling_chunk: &'a mut Chunk
}
#[derive( Copy, Clone, PartialEq, Eq, Debug)]
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
        if !self.had_error {
            disassemble(self.compiling_chunk, "code");
        }
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
        self.compiling_chunk.write_constant(Value::Num(constant), self.prev.line);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpReturn as u8);
    }

    pub fn string(&mut self) {
        
        let st = self.prev.start + 1; // omit "
        let end = self.prev.start + self.prev.length-2; // omit "
        let s = &self.scanner.source[st..=end];
        self.emit_constant_string(s)
    }

    fn emit_constant_string(&mut self, str: &str) {
        let str_val = Value::Obj(Obj::Str(str.to_string()));
        self.compiling_chunk.write_constant(str_val, self.prev.line)
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
        let slice = &self.scanner.source[st..st+len];
        let num: f64 = slice.parse().unwrap();
        self.emit_constant(num.into());
    }
    pub fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }
    pub fn literal(&mut self) {
        match self.prev.token_type {
            TokenType::Nil => {
                self.emit_byte(OpNil as u8);
            }
            TokenType::True => {
                self.emit_byte(OpTrue as u8);
            }
            TokenType::False => {
                self.emit_byte(OpFalse as u8);
            }
            _ => {return;}
        }
    }
    pub fn unary(&mut self) {
        let operator_type = self.prev.token_type;
        // self.expression();
        self.parse_precedence(Unary);
        match operator_type {
            TokenType::Minus => {
                self.emit_byte(OpNegate as u8);
            }
            TokenType::Bang => {
                self.emit_byte(OpNot as u8);
            }
            _ => {return;}
        }
    }
    pub fn binary(&mut self){
        let operator = self.prev.token_type;
        let rule = ParseRule::get_rule(operator);
        let next_rule: u8 = rule.precedence.clone() as u8 + 1;
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
            TokenType::BangEqual => {self.emit_bytes(OpEqual as u8, OpNot as u8);}
            TokenType::EqualEqual => {self.emit_byte(OpEqual as u8);}
            TokenType::Greater => {self.emit_byte(OpGreater as u8);}
            TokenType::GreaterEqual => {self.emit_bytes(OpGreater as u8, OpEqual as u8);}
            TokenType::Less => {self.emit_byte(OpLess as u8);}
            TokenType::LessEqual => {self.emit_bytes(OpLess as u8, OpEqual as u8);}
            _ => {return;}
        }
    }
    pub fn parse_precedence(&mut self, precedence: Precedence){
        // parses any expression greater than equal to the precedence passed to it recursively
        self.advance(); // populates previous
        let prefix_rule = ParseRule::get_rule(self.prev.token_type);
        if let None = prefix_rule.prefix {
            self.error_at_current_with_message("Expect expression.");
            return;
        }
        let f = prefix_rule.prefix.unwrap();
        f(self);
        let precu8 = precedence as u8;
        let mut current_rule = ParseRule::get_rule(self.current.token_type);
        let mut rule_precu8 = current_rule.precedence as u8;
        while precu8 <=  rule_precu8 {
            self.advance();
            let infix_rule = ParseRule::get_rule(self.prev.token_type);
            let f = infix_rule.infix.unwrap();
            f(self);
            current_rule = ParseRule::get_rule(self.current.token_type);
            rule_precu8 = current_rule.precedence as u8;
        }
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

type ParseFn = fn(&mut Parser);

struct ParseRule {
    precedence: Precedence,
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
}

fn parse_grouping(parser: &mut Parser) {
    parser.grouping();
}

fn parse_number(parser: &mut Parser) {
    parser.number();
}

fn parse_literal(parser: &mut Parser) {
    parser.literal();
}

fn parse_unary(parser: &mut Parser) {
    parser.unary();
}

fn parse_binary(parser: &mut Parser) {
    parser.binary();
}

fn parse_string(parser: &mut Parser) {
    parser.string();
}


impl ParseRule {
    const fn new(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: Precedence) -> Self {
        Self { prefix, infix, precedence }
    }
    fn get_rule(token_type: TokenType) -> ParseRule {
        match token_type {
            TokenType::LeftParen => ParseRule::new(Some(parse_grouping), None, Precedence::None),
            TokenType::RightParen => ParseRule::new(None, None, Precedence::None),
            TokenType::LeftBrace => ParseRule::new(None, None, Precedence::None),
            TokenType::RightBrace => ParseRule::new(None, None, Precedence::None),
            TokenType::Comma => ParseRule::new(None, None, Precedence::None),
            TokenType::Dot => ParseRule::new(None, None, Precedence::None),
            TokenType::Minus => ParseRule::new(Some(parse_unary), Some(parse_binary), Precedence::Term),
            TokenType::Plus => ParseRule::new(None, Some(parse_binary), Precedence::Term),
            TokenType::Semicolon => ParseRule::new(None, None, Precedence::None),
            TokenType::Slash => ParseRule::new(None, Some(parse_binary), Precedence::Factor),
            TokenType::Star => ParseRule::new(None, Some(parse_binary), Precedence::Factor),
            TokenType::Bang => ParseRule::new(Some(parse_unary), None, Precedence::None),
            TokenType::BangEqual => ParseRule::new(None, Some(parse_binary), Precedence::Equality),
            TokenType::Equal => ParseRule::new(None, None, Precedence::None),
            TokenType::EqualEqual => ParseRule::new(None, Some(parse_binary), Precedence::Equality),
            TokenType::Greater => ParseRule::new(None, Some(parse_binary), Precedence::Comparison),
            TokenType::GreaterEqual => ParseRule::new(None, Some(parse_binary), Precedence::Comparison),
            TokenType::Less => ParseRule::new(None, Some(parse_binary), Precedence::Comparison),
            TokenType::LessEqual => ParseRule::new(None, Some(parse_binary), Precedence::Comparison),
            TokenType::Identifier => ParseRule::new(None, None, Precedence::None),
            TokenType::String => ParseRule::new(Some(parse_string), None, Precedence::None),
            TokenType::Number => ParseRule::new(Some(parse_number), None, Precedence::None),
            TokenType::And => ParseRule::new(None, None, Precedence::None),
            TokenType::Class => ParseRule::new(None, None, Precedence::None),
            TokenType::Else => ParseRule::new(None, None, Precedence::None),
            TokenType::False => ParseRule::new(Some(parse_literal), None, Precedence::None),
            TokenType::For => ParseRule::new(None, None, Precedence::None),
            TokenType::Fun => ParseRule::new(None, None, Precedence::None),
            TokenType::If => ParseRule::new(None, None, Precedence::None),
            TokenType::Nil => ParseRule::new(Some(parse_literal), None, Precedence::None),
            TokenType::Or => ParseRule::new(None, None, Precedence::None),
            TokenType::Print => ParseRule::new(None, None, Precedence::None),
            TokenType::Return => ParseRule::new(None, None, Precedence::None),
            TokenType::Super => ParseRule::new(None, None, Precedence::None),
            TokenType::This => ParseRule::new(None, None, Precedence::None),
            TokenType::True => ParseRule::new(Some(parse_literal), None, Precedence::None),
            TokenType::Var => ParseRule::new(None, None, Precedence::None),
            TokenType::While => ParseRule::new(None, None, Precedence::None),
            TokenType::Error => ParseRule::new(None, None, Precedence::None),
            TokenType::Eof => ParseRule::new(None, None, Precedence::None),
            TokenType::InitNull => panic!(),
        }
    }
}