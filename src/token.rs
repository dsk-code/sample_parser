#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location(pub usize, pub usize);

impl Location {
    // pub fn new(start: usize, end: usize) -> Self {
    //     Self(start, end)
    // }
    pub fn merge(&self, other: &Location) -> Location {
        use std::cmp::{max, min};
        Location(min(self.0, other.0), max(self.1, other.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annotation<T> {
    value: T,
    loc: Location,
}

impl<T> Annotation<T> {
    pub fn new(value: T, loc: Location) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Number(u64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lparen,
    Rparen,
}

pub type Token = Annotation<TokenKind>;

impl Token {
    pub fn number(n: u64, loc: Location) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    pub fn plus(loc: Location) -> Self {
        Self::new(TokenKind::Plus, loc)
    }

    pub fn minus(loc: Location) -> Self {
        Self::new(TokenKind::Minus, loc)
    }

    pub fn asterisk(loc: Location) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }

    pub fn slash(loc: Location) -> Self {
        Self::new(TokenKind::Slash, loc)
    }

    pub fn lparen(loc: Location) -> Self {
        Self::new(TokenKind::Lparen, loc)
    }

    pub fn rparen(loc: Location) -> Self {
        Self::new(TokenKind::Rparen, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

pub type LexError = Annotation<LexErrorKind>;

impl LexError {
    pub fn invalid_char(c: char, loc: Location) -> Self {
        Self::new(LexErrorKind::InvalidChar(c), loc)
    }

    pub fn eof(loc: Location) -> Self {
        Self::new(LexErrorKind::Eof, loc)
    }
}
