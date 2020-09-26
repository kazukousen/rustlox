use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a String,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        if self.is_digit(c) {
            return self.number();
        }
        if self.is_alpha(c) {
            return self.identifier();
        }

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::SemiColon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '"' => self.string(),
            _ => self.error_token("Unexpected character"),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line = self.line + 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        // A comment goes until the end of the line.
                        while !self.is_at_end() && self.peek() == '\n' {
                            self.advance();
                        }
                    }
                }
                _ => return,
            }
        }
    }

    fn is_digit(&self, c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current);
        self.current = self.current + 1;
        c.expect("Scanner tried to advance to out of bounds character")
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current)
            .expect("Scanner tried to advance to out of bounds character")
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current + 1)
            .expect("Scanner tried to advance to out of bounds character")
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) -> Token {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line = self.line + 1
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("unterminated string");
        }

        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Token {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // look for a fractional part.
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }

        let typ = self.identifier_type();
        self.make_token(typ)
    }

    fn identifier_type(&self) -> TokenType {
        let c = self.source.chars().nth(self.start)
            .expect("Scanner tried to peek identifier out of bounds character");

        match c {
            'i' => self.check_keyword(1, "f", TokenType::If),
            'f' => if self.current - self.start > 1 {
                match self.source.chars().nth(self.start + 1)
                    .expect("Scanner tried to peek identifier out of bounds character") {
                    'a' => self.check_keyword(2, "lse", TokenType::False),
                    'o' => self.check_keyword(2, "r", TokenType::For),
                    'u' => self.check_keyword(2, "n", TokenType::Fun),
                    _ => TokenType::Identifier,
                }
            } else {
                TokenType::Identifier
            },
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(&self, start: usize, rest: &str, typ: TokenType) -> TokenType {

        if self.current - self.start != start + rest.len() {
            return TokenType::Identifier;
        }

        let source = &self.source[self.start + start..self.current];
        if source == rest {
            return typ;
        }

        TokenType::Identifier
    }

    fn make_token(&self, typ: TokenType) -> Token {
        Token {
            typ,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            source: self.source,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            typ: TokenType::Error,
            start: 0,
            length: message.len(),
            line: self.line,
            source: self.source,
        }
    }
}
