use crate::span::Span;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(len: usize, kind: TokenKind) -> Self {
        Self { len, kind }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Multi Character
    /// "# THIS IS A LINE COMMENT"
    LineComment,
    /// Any whitespace character sequence except "\n"
    Whitespace,
    /// Labels
    Identifier,
    /// NOR | LOAD
    Keyword(Keyword),
    /// "5" | "0b011101" | "0xD"
    Numeric { base: Base, prefix_len: usize },
    /// "'Z'"Base
    Character { terminated: bool },

    // Single (me too bitch) Character (oh... sorry)
    /// A | B | C
    Register(Register),
    /// "\n"
    NewLine,
    /// ":"
    Colon,
    /// "="
    Eq,
    /// "|"
    Or,
    /// "!"
    Bang,
    /// ">"
    Greater,
    /// "<"
    Less,
    /// "("
    OpenParen,
    /// ")"
    CloseParen,
    /// "["
    OpenBracket,
    /// "]"
    CloseBracket,

    // Special
    /// Unknown token
    Unknown,
    /// End of file
    Eof,
}

#[derive(Debug, PartialEq, EnumString, Clone)]
pub enum Keyword {
    NOR,
    PC,
    LOAD,
    STORE,
    LABEL,
    SET,
    LIH,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Base {
    Binary,
    Decimal,
    Hex,
}

#[derive(Debug, PartialEq, EnumString, Clone)]
pub enum Register {
    A,
    B,
    C,
}
