use std::str::Chars;

use crate::tokens::{Base, Token, TokenKind};

const EOF_CHAR: char = '\0';

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::from(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    })
}

#[derive(Debug)]
pub struct Cursor<'a> {
    remaining: usize,
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }

    pub fn is_eof(&self) -> bool {
        self.as_str().is_empty()
    }

    pub fn token_len(&self) -> usize {
        self.remaining - self.as_str().len()
    }

    pub fn set_remaining(&mut self) {
        self.remaining = self.as_str().len()
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
            None => return Token::new(0, TokenKind::Eof),
        };

        let kind = match first {
            c if is_ident(c) => self.consume_identifier(),

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

            c if c.is_whitespace() && c != '\n' => TokenKind::Whitespace,
            _ => TokenKind::Unknown,
        };

        let len = self.token_len();
        self.set_remaining();
        Token::new(len, kind)
    }

    fn consume_line_comment(&mut self) -> TokenKind {
        self.bump_while(|c| c != '\n');
        TokenKind::LineComment
    }

    fn consume_number(&mut self, first: char) -> TokenKind {
        let (base, prefix_len, radix) = if first == '0' {
            match self.first() {
                'b' => {
                    self.bump();
                    (Base::Binary, 2, 2)
                }
                'x' => {
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

    fn consume_identifier(&mut self) -> TokenKind {
        self.bump_while(is_ident);
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
            remaining: value.len(),
            chars: value.chars(),
        }
    }
}

fn is_ident(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_creation() {
        let input = "test input";
        let cursor: Cursor = input.into();
        assert_eq!(cursor.remaining, input.len());
        assert_eq!(cursor.as_str(), input);
    }

    #[test]
    fn test_is_eof() {
        let mut cursor: Cursor = "a".into();
        assert!(!cursor.is_eof());
        cursor.bump();
        assert!(cursor.is_eof());
    }

    #[test]
    fn test_first() {
        let cursor: Cursor = "abc".into();
        assert_eq!(cursor.first(), 'a');
    }

    #[test]
    fn test_bump() {
        let mut cursor: Cursor = "abc".into();
        assert_eq!(cursor.bump(), Some('a'));
        assert_eq!(cursor.bump(), Some('b'));
        assert_eq!(cursor.bump(), Some('c'));
        assert_eq!(cursor.bump(), None);
    }

    #[test]
    fn test_bump_while() {
        let mut cursor: Cursor = "aaabbb".into();
        cursor.bump_while(|c| c == 'a');
        assert_eq!(cursor.as_str(), "bbb");
    }

    #[test]
    fn test_advance_token_identifier() {
        let mut cursor: Cursor = "A_bc123".into();
        let token = cursor.advance_token();
        assert_eq!(token.kind, TokenKind::Identifier);
        assert_eq!(token.len, 4);
    }

    #[test]
    fn test_advance_token_numeric() {
        let mut cursor: Cursor = "12_3abc".into();
        let token = cursor.advance_token();
        assert!(matches!(
            token.kind,
            TokenKind::Numeric {
                base: Base::Decimal,
                prefix_len: 0
            }
        ));
        assert_eq!(token.len, 4);
    }

    #[test]
    fn test_advance_token_hex() {
        let mut cursor: Cursor = "0xFF_abc".into();
        let token = cursor.advance_token();
        assert!(matches!(
            token.kind,
            TokenKind::Numeric {
                base: Base::Hex,
                prefix_len: 2
            }
        ));
        assert_eq!(token.len, 8);
    }

    #[test]
    fn test_advance_token_binary() {
        let mut cursor: Cursor = "0b10_1010abc".into();
        let token = cursor.advance_token();
        assert!(matches!(
            token.kind,
            TokenKind::Numeric {
                base: Base::Binary,
                prefix_len: 2
            }
        ));
        assert_eq!(token.len, 9);
    }

    #[test]
    fn test_advance_token_character() {
        let mut cursor: Cursor = "'a'bc".into();
        let token = cursor.advance_token();
        assert!(matches!(
            token.kind,
            TokenKind::Character { terminated: true }
        ));
        assert_eq!(token.len, 3);
    }

    #[test]
    fn test_advance_token_unterminated_character() {
        let mut cursor: Cursor = "'abc".into();
        let token = cursor.advance_token();
        assert!(matches!(
            token.kind,
            TokenKind::Character { terminated: false }
        ));
        assert_eq!(token.len, 2);
    }

    #[test]
    fn test_advance_token_line_comment() {
        let mut cursor: Cursor = "# This is a comment\nabc".into();
        let token = cursor.advance_token();
        assert_eq!(token.kind, TokenKind::LineComment);
        assert_eq!(token.len, 19);
    }

    #[test]
    fn test_advance_token_symbols() {
        let input = ":=|!><()[]";
        let expected = vec![
            TokenKind::Colon,
            TokenKind::Eq,
            TokenKind::Or,
            TokenKind::Bang,
            TokenKind::Greater,
            TokenKind::Less,
            TokenKind::OpenParen,
            TokenKind::CloseParen,
            TokenKind::OpenBracket,
            TokenKind::CloseBracket,
        ];
        let mut cursor: Cursor = input.into();
        for expected_kind in expected {
            let token = cursor.advance_token();
            assert_eq!(token.kind, expected_kind);
            assert_eq!(token.len, 1);
        }
    }

    #[test]
    fn test_advance_token_whitespace() {
        let mut cursor: Cursor = " \t\n".into();
        let token = cursor.advance_token();
        assert_eq!(token.kind, TokenKind::Whitespace);
        assert_eq!(token.len, 1);
    }

    #[test]
    fn test_advance_token_unknown() {
        let mut cursor: Cursor = "@abc".into();
        let token = cursor.advance_token();
        assert_eq!(token.kind, TokenKind::Unknown);
        assert_eq!(token.len, 1);
    }

    #[test]
    fn test_advance_token_eof() {
        let mut cursor: Cursor = "".into();
        let token = cursor.advance_token();
        assert_eq!(token.kind, TokenKind::Eof);
        assert_eq!(token.len, 0);
    }
}
