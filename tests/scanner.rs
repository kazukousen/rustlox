use rustlox::*;

#[test]
fn run_empty() {
    let source = "".to_string();
    let mut sc = Scanner::new(&source);

    let token = sc.scan_token();

    assert_eq!(TokenType::Eof, token.typ);
}
#[test]
fn run_arithmetic() {
    let source = "1.2 + 3.8 == 5".to_string();
    let mut sc = Scanner::new(&source);

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