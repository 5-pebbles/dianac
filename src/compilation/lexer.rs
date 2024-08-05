use std::str::Chars;

use crate::compilation::{
    span::Span,
    tokens::{Base, Keyword, Register, Token, TokenKind},
};

const EOF_CHAR: char = '\0';

#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    len: usize,
    progress: usize,
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub fn tokenize(mut self) -> impl Iterator<Item = Token> + 'a {
        std::iter::from_fn(move || {
            let token = self.advance_token();
            if token.kind != TokenKind::Eof {
                Some(token)
            } else {
                None
            }
        })
    }

    pub fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }

    pub fn is_eof(&self) -> bool {
        self.as_str().is_empty()
    }

    pub fn token_span(&self) -> Span {
        Span::new(self.progress, self.len - self.as_str().len())
    }

    pub fn set_progress(&mut self) {
        self.progress = self.len - self.as_str().len()
    }

    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn bump_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    pub fn advance_token(&mut self) -> Token {
        let first = match self.bump() {
            Some(value) => value,
            None => return Token::new(self.token_span(), TokenKind::Eof),
        };

        let kind = match first {
            c if c.is_alphabetic() || c == '_' => self.consume_identifier_or_keyword_or_register(c),

            c @ '0'..='9' => self.consume_number(c),
            '\'' => self.consume_character_literal(),

            '#' => self.consume_line_comment(),
            '\n' => TokenKind::NewLine,
            ':' => TokenKind::Colon,
            '=' => TokenKind::Eq,
            '|' => TokenKind::Or,
            '!' => TokenKind::Bang,
            '>' => TokenKind::Greater,
            '<' => TokenKind::Less,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,

            c if c.is_whitespace() && c != '\n' => {
                self.set_progress();
                return self.advance_token();
            }
            _ => TokenKind::Unknown,
        };

        let span = self.token_span();
        self.set_progress();
        Token::new(span, kind)
    }

    fn consume_line_comment(&mut self) -> TokenKind {
        self.bump_while(|c| c != '\n');
        TokenKind::LineComment
    }

    fn consume_number(&mut self, first: char) -> TokenKind {
        let (base, prefix_len, radix) = if first == '0' {
            match self.first() {
                'b' | 'B' => {
                    self.bump();
                    (Base::Binary, 2, 2)
                }
                'x' | 'X' => {
                    self.bump();
                    (Base::Hex, 2, 16)
                }
                _ => (Base::Decimal, 0, 10),
            }
        } else {
            (Base::Decimal, 0, 10)
        };

        self.consume_suffix(radix);
        TokenKind::Numeric { base, prefix_len }
    }

    fn consume_identifier_or_keyword_or_register(&mut self, first: char) -> TokenKind {
        let mut ident = first.to_string();
        self.bump_while(|c| {
            if is_ident(c) {
                ident.push(c);
                true
            } else {
                false
            }
        });

        if let Ok(register) = ident.parse::<Register>() {
            return TokenKind::Register(register);
        }

        if let Ok(keyword) = ident.parse::<Keyword>() {
            return TokenKind::Keyword(keyword);
        }

        TokenKind::Identifier
    }

    fn consume_suffix(&mut self, radix: u32) {
        self.bump_while(|c| c.is_digit(radix) || c == '_');
    }

    fn consume_character_literal(&mut self) -> TokenKind {
        self.bump();
        let terminated = self.first() == '\'';
        if terminated {
            self.bump();
        }
        TokenKind::Character { terminated }
    }
}

impl<'a> From<&'a str> for Cursor<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            len: value.len(),
            progress: 0,
            chars: value.chars(),
        }
    }
}

fn is_ident(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{Base, Keyword, Register, TokenKind};

    #[test]
    fn test_tokenize_basic() {
        let input = "A = 42";
        let tokens: Vec<TokenKind> = Cursor::from(input).tokenize().map(|t| t.kind).collect();
        assert_eq!(tokens, vec![
            TokenKind::Register(Register::A),
            TokenKind::Whitespace,
            TokenKind::Eq,
            TokenKind::Whitespace,
            TokenKind::Numeric {
                base: Base::Decimal,
                prefix_len: 0
            },
        ]);
    }

    #[test]
    fn test_tokenize_keywords_and_identifiers() {
        let input = "LOAD foo BAR";
        let tokens: Vec<TokenKind> = Cursor::from(input).tokenize().map(|t| t.kind).collect();
        assert_eq!(tokens, vec![
            TokenKind::Keyword(Keyword::Load),
            TokenKind::Whitespace,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::Identifier,
        ]);
    }

    #[test]
    fn test_tokenize_numbers() {
        let input = "10 0xFF 0b1010";
        let tokens: Vec<TokenKind> = Cursor::from(input).tokenize().map(|t| t.kind).collect();
        assert_eq!(tokens, vec![
            TokenKind::Numeric {
                base: Base::Decimal,
                prefix_len: 0
            },
            TokenKind::Whitespace,
            TokenKind::Numeric {
                base: Base::Hex,
                prefix_len: 2
            },
            TokenKind::Whitespace,
            TokenKind::Numeric {
                base: Base::Binary,
                prefix_len: 2
            },
        ]);
    }

    #[test]
    fn test_tokenize_symbols_and_comments() {
        let input = "A > B # Comment\n:";
        let tokens: Vec<TokenKind> = Cursor::from(input).tokenize().map(|t| t.kind).collect();
        assert_eq!(tokens, vec![
            TokenKind::Register(Register::A),
            TokenKind::Whitespace,
            TokenKind::Greater,
            TokenKind::Whitespace,
            TokenKind::Register(Register::B),
            TokenKind::Whitespace,
            TokenKind::LineComment,
            TokenKind::NewLine,
            TokenKind::Colon,
        ]);
    }

    #[test]
    fn test_tokenize_character_literals() {
        let input = "'a' '\n' 'x";
        let tokens: Vec<TokenKind> = Cursor::from(input).tokenize().map(|t| t.kind).collect();
        assert_eq!(tokens, vec![
            TokenKind::Character { terminated: true },
            TokenKind::Whitespace,
            TokenKind::Character { terminated: true },
            TokenKind::Whitespace,
            TokenKind::Character { terminated: false },
        ]);
    }
}
