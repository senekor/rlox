use crate::{error, token::Token, token_type::TokenType as T};

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
            literal: (),
            line: self.line,
        });
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(T::LeftParen, ()),
            ')' => self.add_token(T::RightParen, ()),
            '{' => self.add_token(T::LeftBrace, ()),
            '}' => self.add_token(T::RightBrace, ()),
            ',' => self.add_token(T::Comma, ()),
            '.' => self.add_token(T::Dot, ()),
            '-' => self.add_token(T::Minus, ()),
            '+' => self.add_token(T::Plus, ()),
            ';' => self.add_token(T::Semicolon, ()),
            '*' => self.add_token(T::Star, ()),
            '!' => {
                let token = if self.match_char('=') {
                    T::BangEqual
                } else {
                    T::Bang
                };
                self.add_token(token, ())
            }
            '=' => {
                let token = if self.match_char('=') {
                    T::EqualEqual
                } else {
                    T::Equal
                };
                self.add_token(token, ())
            }
            '<' => {
                let token = if self.match_char('=') {
                    T::LessEqual
                } else {
                    T::Less
                };
                self.add_token(token, ())
            }
            '>' => {
                let token = if self.match_char('=') {
                    T::GreaterEqual
                } else {
                    T::Greater
                };
                self.add_token(token, ())
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(T::Slash, ());
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
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

    fn add_token(&mut self, token_type: T, literal: ()) {
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
}
