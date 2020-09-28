use rustlox::*;

#[test]
fn run_empty() {
    let source = "if false {}";
    let mut sc = Scanner::new(source);

    let token = sc.scan_token();
    assert_eq!(TokenType::If, token.typ);
    let token = sc.scan_token();
    assert_eq!(TokenType::False, token.typ);
    let token = sc.scan_token();
    assert_eq!(TokenType::LeftBrace, token.typ);
    let token = sc.scan_token();
    assert_eq!(TokenType::RightBrace, token.typ);
    let token = sc.scan_token();
    assert_eq!(TokenType::Eof, token.typ);
}

#[test]
fn run_arithmetic() {
    let source = "1.2 + 3.8 == 5";
    let mut sc = Scanner::new(source);

    let token = sc.scan_token();
    assert_eq!(TokenType::Number, token.typ);
    let token = sc.scan_token();
    assert_eq!(TokenType::Plus, token.typ);
    let token = sc.scan_token();
    assert_eq!(TokenType::Number, token.typ);
    let token = sc.scan_token();
    assert_eq!(TokenType::EqualEqual, token.typ);
    let token = sc.scan_token();
    assert_eq!(TokenType::Number, token.typ);
    let token = sc.scan_token();
    assert_eq!(TokenType::Eof, token.typ);
}

#[test]
fn run_skip_comment() {
    let source = r#"
// skip this line
print 1 + 2;
// skip this line
"#;
    let mut sc = Scanner::new(source);
    let token = sc.scan_token();
    assert_eq!(TokenType::Print, token.typ);
    assert_eq!(2, token.line);
    assert_eq!("print", token.source);
    let token = sc.scan_token();
    assert_eq!(TokenType::Number, token.typ);
    assert_eq!(2, token.line);
    assert_eq!("1", token.source);
    let token = sc.scan_token();
    assert_eq!(TokenType::Plus, token.typ);
    assert_eq!(2, token.line);
    assert_eq!("+", token.source);
    let token = sc.scan_token();
    assert_eq!(TokenType::Number, token.typ);
    assert_eq!(2, token.line);
    assert_eq!("2", token.source);
    let token = sc.scan_token();
    assert_eq!(TokenType::SemiColon, token.typ);
    assert_eq!(2, token.line);
    assert_eq!(";", token.source);
    let token = sc.scan_token();
    assert_eq!(TokenType::Eof, token.typ);
    assert_eq!(4, token.line);
}
