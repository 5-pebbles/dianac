use strum::{Display as EnumDisplay, EnumString};

use crate::compilation::span::Span;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(span: Span, kind: TokenKind) -> Self {
        Self { span, kind }
    }
}

#[derive(Debug, PartialEq, Clone, EnumDisplay)]
pub enum TokenKind {
    // Multi Character
    /// "# THIS IS A LINE COMMENT"
    LineComment,
    /// Labels
    Identifier,
    /// NOR | LOAD
    Keyword(Keyword),
    /// "5" | "0b011101" | "0xD"
    Numeric { base: Base, prefix_len: usize },
    /// "'Z'"
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
#[strum(serialize_all = "UPPERCASE")]
pub enum Keyword {
    // Logic
    Not,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Nxor,
    // Shift and Rotate
    Rol,
    Ror,
    Shl,
    Shr,
    // Arithmetic
    Add,
    // Memory
    Set,
    Mov,
    Lod,
    Sto,
    // Jump
    Pc,
    Lab,
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
