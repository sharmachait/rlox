use rlox::lexer::{
    lexer::Scanner,
    token_type::TokenType
};
#[test]
fn lexer_keywords_and(){
    let source = &("and".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::And);
}
#[test]
fn lexer_keywords_class(){
    let source = &("class".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Class);
}
#[test]
fn lexer_keywords_else(){
    let source = &("else".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Else);
}
#[test]
fn lexer_keywords_if(){
    let source = &("if".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::If);
}
#[test]
fn lexer_keywords_nil(){
    let source = &("nil".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Nil);
}
#[test]
fn lexer_keywords_or(){
    let source = &("or".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Or);
}
#[test]
fn lexer_keywords_print(){
    let source = &("print".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Print);
}
#[test]
fn lexer_keywords_return(){
    let source = &("return".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Return);
}
#[test]
fn lexer_keywords_super(){
    let source = &("super".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Super);
}
#[test]
fn lexer_keywords_var(){
    let source = &("var".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Var);
}
#[test]
fn lexer_keywords_while(){
    let source = &("while".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::While);
}
#[test]
fn lexer_keywords_this(){
    let source = &("this".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::This);
}
#[test]
fn lexer_keywords_fun(){
    let source = &("fun".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Fun);
}
#[test]
fn lexer_keywords_true(){
    let source = &("true".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::True);
}
#[test]
fn lexer_keywords_false(){
    let source = &("false".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::False);
}
#[test]
fn lexer_keywords_for(){
    let source = &("for".to_string());
    let mut scanner = Scanner::new(source);

    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::For);
}

// Test multi-character operators
#[test]
fn lexer_bang_equal(){
    let source = &("!=".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::BangEqual);
}

#[test]
fn lexer_equal_equal(){
    let source = &("==".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::EqualEqual);
}

#[test]
fn lexer_less_equal(){
    let source = &("<=".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::LessEqual);
}

#[test]
fn lexer_greater_equal(){
    let source = &(">=".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::GreaterEqual);
}

// Test single character tokens
#[test]
fn lexer_single_char_bang(){
    let source = &("!".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Bang);
}

#[test]
fn lexer_single_char_equal(){
    let source = &("=".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Equal);
}

#[test]
fn lexer_single_char_less(){
    let source = &("<".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Less);
}

#[test]
fn lexer_single_char_greater(){
    let source = &(">".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Greater);
}

#[test]
fn lexer_left_paren(){
    let source = &("(".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::LeftParen);
}

#[test]
fn lexer_right_paren(){
    let source = &(")".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::RightParen);
}

#[test]
fn lexer_left_brace(){
    let source = &("{".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::LeftBrace);
}

#[test]
fn lexer_right_brace(){
    let source = &("}".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::RightBrace);
}

#[test]
fn lexer_semicolon(){
    let source = &(";".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Semicolon);
}

#[test]
fn lexer_comma(){
    let source = &(",".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Comma);
}

#[test]
fn lexer_dot(){
    let source = &(".".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Dot);
}

#[test]
fn lexer_minus(){
    let source = &("-".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Minus);
}

#[test]
fn lexer_plus(){
    let source = &("+".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Plus);
}

#[test]
fn lexer_slash(){
    let source = &("/".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Slash);
}

#[test]
fn lexer_star(){
    let source = &("*".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Star);
}

// Test numbers
#[test]
fn lexer_integer(){
    let source = &("123".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Number);
}

#[test]
fn lexer_decimal(){
    let source = &("123.456".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Number);
}

#[test]
fn lexer_zero(){
    let source = &("0".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Number);
}

// Test strings
#[test]
fn lexer_string(){
    let source = &("\"hello world\"".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::String);
}

#[test]
fn lexer_empty_string(){
    let source = &("\"\"".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::String);
}

// Test identifiers
#[test]
fn lexer_identifier(){
    let source = &("myVariable".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Identifier);
}

#[test]
fn lexer_identifier_with_underscore(){
    let source = &("_myVar".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Identifier);
}

#[test]
fn lexer_identifier_with_numbers(){
    let source = &("var123".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Identifier);
}

// Test keyword prefixes (shouldn't match keywords)
#[test]
fn lexer_identifier_and_prefix(){
    let source = &("android".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Identifier);
}

#[test]
fn lexer_identifier_if_prefix(){
    let source = &("ifTrue".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Identifier);
}

#[test]
fn lexer_identifier_for_prefix(){
    let source = &("format".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Identifier);
}

// Test EOF
#[test]
fn lexer_eof(){
    let source = &("".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Eof);
}

// Test whitespace skipping
#[test]
fn lexer_skip_whitespace(){
    let source = &("   123".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Number);
}

#[test]
fn lexer_skip_tabs(){
    let source = &("\t\t123".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Number);
}

// Test comments
#[test]
fn lexer_skip_comment(){
    let source = &("// this is a comment\n123".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Number);
}

#[test]
fn lexer_comment_at_end(){
    let source = &("123 // comment".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Number);
    let token2 = scanner.scan_token();
    assert_eq!(token2.token_type, TokenType::Eof);
}

// Test multiple tokens
#[test]
fn lexer_multiple_tokens(){
    let source = &("var x = 123;".to_string());
    let mut scanner = Scanner::new(source);

    let token1 = scanner.scan_token();
    assert_eq!(token1.token_type, TokenType::Var);

    let token2 = scanner.scan_token();
    assert_eq!(token2.token_type, TokenType::Identifier);

    let token3 = scanner.scan_token();
    assert_eq!(token3.token_type, TokenType::Equal);

    let token4 = scanner.scan_token();
    assert_eq!(token4.token_type, TokenType::Number);

    let token5 = scanner.scan_token();
    assert_eq!(token5.token_type, TokenType::Semicolon);
}

// Test line tracking
#[test]
fn lexer_line_tracking(){
    let source = &("123\n456".to_string());
    let mut scanner = Scanner::new(source);

    let token1 = scanner.scan_token();
    assert_eq!(token1.line, 1);

    let token2 = scanner.scan_token();
    assert_eq!(token2.line, 2);
}

// Test error cases
#[test]
fn lexer_unexpected_character(){
    let source = &("@".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Error);
}

#[test]
fn lexer_unterminated_string(){
    let source = &("\"hello".to_string());
    let mut scanner = Scanner::new(source);
    let token = scanner.scan_token();
    assert_eq!(token.token_type, TokenType::Error);
}