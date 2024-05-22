use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}
impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>, // idk what this is supposed to be yet, Box<dyn Any> cannot be Clone.
    pub line: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
