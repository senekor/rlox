use crate::{
    error,
    token::{Literal, Token},
    token_type::TokenType as T,
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: T::Eof,
            lexeme: "".into(),
            literal: None,
            line: self.line,
        });
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(T::LeftParen, None),
            ')' => self.add_token(T::RightParen, None),
            '{' => self.add_token(T::LeftBrace, None),
            '}' => self.add_token(T::RightBrace, None),
            ',' => self.add_token(T::Comma, None),
            '.' => self.add_token(T::Dot, None),
            '-' => self.add_token(T::Minus, None),
            '+' => self.add_token(T::Plus, None),
            ';' => self.add_token(T::Semicolon, None),
            '*' => self.add_token(T::Star, None),
            '!' => {
                let token = if self.match_char('=') {
                    T::BangEqual
                } else {
                    T::Bang
                };
                self.add_token(token, None)
            }
            '=' => {
                let token = if self.match_char('=') {
                    T::EqualEqual
                } else {
                    T::Equal
                };
                self.add_token(token, None)
            }
            '<' => {
                let token = if self.match_char('=') {
                    T::LessEqual
                } else {
                    T::Less
                };
                self.add_token(token, None)
            }
            '>' => {
                let token = if self.match_char('=') {
                    T::GreaterEqual
                } else {
                    T::Greater
                };
                self.add_token(token, None)
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(T::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            _ => error(self.line, "Unexpected character.".into()),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let res = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        res
    }

    fn add_token(&mut self, token_type: T, literal: Option<Literal>) {
        let text: String = self.source[self.start..self.current].into();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        })
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.".into());
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[self.start + 1..self.current - 1].into();
        self.add_token(T::String, Some(value));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .parse::<f64>()
            .unwrap()
            .into();
        self.add_token(T::Number, Some(value));
    }
}
