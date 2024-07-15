#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(len: usize, kind: TokenKind) -> Self {
        Self { len, kind }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Multi Character
    /// "# THIS IS A LINE COMMENT"
    LineComment,
    /// Any whitespace character sequence except "\n"
    Whitespace,
    /// Label | Register | Keyword
    Identifier,
    /// "5" | "0b011101" | "0xD"
    Numeric { base: Base, prefix_len: usize },
    /// "'Z'"Base
    Character { terminated: bool },

    // Separators
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

#[derive(Debug, PartialEq)]
pub enum Base {
    Binary,
    Decimal,
    Hex,
}
