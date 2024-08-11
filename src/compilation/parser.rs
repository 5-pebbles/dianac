use std::num::IntErrorKind;

use arbitrary_int::u6;

use crate::{
    character_encoding::encode_character,
    compilation::{
        diagnostic::{DiagKind, DiagLevel, Diagnostic},
        handlers,
        ir::{AddressTuple, Either, Immediate, Ir, IrRegister},
        lexer::Cursor,
        span::Span,
        tokens::{Base, Keyword, Register, Token, TokenKind},
    },
};

macro_rules! match_token_kind {
    ($pattern:pat) => {
        Token { kind: $pattern, .. }
    };
}

fn unexpected_token_error(found: Token, expected: &'static str) -> Diagnostic {
    Diagnostic {
        level: DiagLevel::Fatal,
        span: found.span,
        kind: DiagKind::UnexpectedToken {
            found: found.kind,
            expected,
        },
    }
}

pub fn u6_from_str_radix(str: &str, radix: u32) -> Result<u6, IntErrorKind> {
    u6::try_new(u8::from_str_radix(str, radix).map_err(|e| e.kind().clone())?)
        .map_err(|_| IntErrorKind::PosOverflow)
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    cursor: Cursor<'a>,
    raw: &'a str,
    ir: Vec<Ir<'a>>,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Parser<'a> {
    pub fn parse(mut self) -> (Vec<Ir<'a>>, Vec<Diagnostic>) {
        while !self.cursor.is_eof() {
            self.advance_ir();
        }

        (self.ir, self.diagnostics)
    }

    pub fn advance_ir(&mut self) {
        // TODO move to next line on error
        let keyword = match self.cursor.advance_token() {
            match_token_kind!(TokenKind::NewLine) => return,
            match_token_kind!(TokenKind::LineComment) => {
                self.parse_end_of_line();
                return;
            }
            match_token_kind!(TokenKind::Keyword(keyword)) => keyword,
            token => {
                self.diagnostics
                    .push(unexpected_token_error(token, "Keyword | Comment | NewLine"));
                return;
            }
        };

        let mut parse_helper = || -> Result<Vec<Ir>, Diagnostic> {
            Ok(match keyword {
                // Logic
                Keyword::Not => handlers::not(self.parse_register()?),
                Keyword::And => handlers::and(self.parse_register()?, self.parse_either()?),
                Keyword::Nand => handlers::nand(self.parse_register()?, self.parse_either()?),
                Keyword::Or => handlers::or(self.parse_register()?, self.parse_either()?),
                Keyword::Nor => handlers::nor(self.parse_register()?, self.parse_either()?),
                Keyword::Xor => handlers::xor(self.parse_register()?, self.parse_either()?),
                Keyword::Nxor => handlers::nxor(self.parse_register()?, self.parse_either()?),
                // Shift and Rotate
                Keyword::Rol => handlers::rol(self.parse_either()?),
                Keyword::Ror => handlers::ror(self.parse_either()?),
                Keyword::Shl => handlers::shl(self.parse_either()?),
                Keyword::Shr => handlers::shr(self.parse_either()?),
                // Arithmetic
                Keyword::Add => handlers::add(self.parse_register()?, self.parse_either()?),
                Keyword::Sub => handlers::sub(self.parse_register()?, self.parse_either()?),
                // Memory
                Keyword::Set => handlers::set(self.parse_immediate()?),
                Keyword::Mov => handlers::mov(self.parse_register()?, self.parse_either()?),
                Keyword::Lod => handlers::lod(self.parse_address_tuple()?),
                Keyword::Sto => handlers::sto(self.parse_address_tuple()?),
                // Jump
                Keyword::Pc => handlers::pc(self.parse_address_tuple()?),
                Keyword::Lab => {
                    let (label, span) = self.parse_identifier()?;
                    handlers::lab(label, span)
                }
                // Miscellaneous
                Keyword::HLT => handlers::hlt(),
            })
        };

        match parse_helper() {
            Ok(ir) => self.ir.extend(ir),
            Err(diagnostic) => self.diagnostics.push(diagnostic),
        };

        // TODO allow trailing comments
        self.parse_end_of_line()
    }

    pub fn parse_identifier(&mut self) -> Result<(&'a str, Span), Diagnostic> {
        match self.cursor.advance_token() {
            token @ match_token_kind!(TokenKind::Identifier) => {
                Ok((&self.raw[token.span.as_range()], token.span))
            }
            unexpected => Err(unexpected_token_error(unexpected, "Identifier")),
        }
    }

    pub fn parse_address_tuple(&mut self) -> Result<AddressTuple<'a>, Diagnostic> {
        let mut clone = self.cursor.clone();
        if clone.advance_token().kind == TokenKind::Identifier
            && clone.advance_token().kind != TokenKind::Colon
        {
            let token = self.cursor.advance_token();
            let span = token.span;
            let label = &self.raw[span.as_range()];
            Ok(AddressTuple(
                Either::Immediate(Immediate::LabelP0(label, span.clone())),
                Either::Immediate(Immediate::LabelP1(label, span)),
            ))
        } else {
            Ok(AddressTuple(self.parse_either()?, self.parse_either()?))
        }
    }

    pub fn parse_either(&mut self) -> Result<Either<'a>, Diagnostic> {
        Ok(
            if let TokenKind::Register(_) = self.cursor.clone().advance_token().kind {
                Either::Register(self.parse_register()?)
            } else {
                Either::Immediate(self.parse_immediate()?)
            },
        )
    }

    pub fn parse_register(&mut self) -> Result<IrRegister, Diagnostic> {
        match self.cursor.advance_token() {
            match_token_kind!(TokenKind::Register(register)) => Ok(match register {
                Register::A => IrRegister::A,
                Register::B => IrRegister::B,
                Register::C => IrRegister::C,
            }),
            unexpected => Err(unexpected_token_error(unexpected, "Register")),
        }
    }

    pub fn parse_immediate(&mut self) -> Result<Immediate<'a>, Diagnostic> {
        match self.cursor.advance_token() {
            match_token_kind!(TokenKind::OpenParen) => {
                let immediate = self.parse_block()?;
                match self.cursor.advance_token() {
                    match_token_kind!(TokenKind::CloseParen) => Ok(immediate),
                    unexpected => Err(unexpected_token_error(unexpected, "CloseParen")),
                }
            }
            match_token_kind!(TokenKind::Bang) => {
                Ok(Immediate::Not(Box::new(self.parse_immediate()?)))
            }
            token @ match_token_kind!(TokenKind::Identifier) => self.parse_label(token),
            ref token @ match_token_kind!(TokenKind::Numeric { ref base, ref prefix_len }) => {
                self.parse_numeric(&token.span, base, prefix_len)
            }
            ref token @ match_token_kind!(TokenKind::Character { ref terminated }) => {
                self.parse_character(&token.span, &terminated)
            }
            unexpected => Err(unexpected_token_error(
                unexpected,
                "OpenParen | Bang | Numeric | Label | Character",
            )),
        }
    }

    fn parse_block(&mut self) -> Result<Immediate<'a>, Diagnostic> {
        let immediate = self.parse_immediate()?;
        Ok(match self.cursor.clone().advance_token() {
            match_token_kind!(TokenKind::Or) => {
                self.cursor.advance_token();
                Immediate::Or(Box::new(immediate), Box::new(self.parse_immediate()?))
            }
            _ => immediate,
        })
    }

    fn parse_numeric(
        &self,
        span: &Span,
        base: &Base,
        prefix_len: &usize,
    ) -> Result<Immediate<'a>, Diagnostic> {
        let radix = match base {
            Base::Binary => 2,
            Base::Decimal => 10,
            Base::Hex => 16,
        };

        let numeric = u6_from_str_radix(&self.raw[span.start + prefix_len..span.end], radix)
            .map_err(|e| Diagnostic {
                level: DiagLevel::Fatal,
                span: span.clone(),
                kind: DiagKind::ParseImmediate(e),
            })?;

        Ok(Immediate::Constant(numeric))
    }

    fn parse_label(&mut self, first: Token) -> Result<Immediate<'a>, Diagnostic> {
        match self.cursor.advance_token() {
            match_token_kind!(TokenKind::Colon) => (),
            unexpected => return Err(unexpected_token_error(unexpected, "Colon")),
        };

        let num = self.cursor.advance_token();
        match &self.raw[num.span.as_range()] {
            "0" => Ok(Immediate::LabelP0(
                &self.raw[first.span.as_range()],
                first.span.merge(num.span),
            )),
            "1" => Ok(Immediate::LabelP1(
                &self.raw[first.span.as_range()],
                first.span.merge(num.span),
            )),
            _ => Err(unexpected_token_error(num, "Numeric(Decimal(`0` | `1`))")),
        }
    }

    pub fn parse_character(
        &mut self,
        span: &Span,
        terminated: &bool,
    ) -> Result<Immediate<'a>, Diagnostic> {
        if !terminated {
            return Err(Diagnostic {
                level: DiagLevel::Fatal,
                span: span.clone(),
                kind: DiagKind::IncompleteCharacter,
            });
        }

        let character = self.raw[span.as_range()].chars().nth(1).unwrap();

        encode_character(&character)
            .map(|numeric| Immediate::Constant(*numeric))
            .ok_or(Diagnostic {
                level: DiagLevel::Fatal,
                span: span.clone(),
                kind: DiagKind::UnsupportedCharacter(character),
            })
    }

    pub fn parse_end_of_line(&mut self) {
        match self.cursor.clone().advance_token() {
            match_token_kind!(TokenKind::NewLine | TokenKind::Eof) => {
                self.cursor.advance_token();
                ()
            }
            token => self
                .diagnostics
                .push(unexpected_token_error(token, "NewLine | Eof")),
        }
    }
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            cursor: Cursor::from(value),
            raw: value,
            ir: Vec::new(),
            diagnostics: Vec::new(),
        }
    }
}
