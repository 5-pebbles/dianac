use std::num::IntErrorKind;

use arbitrary_int::u6;

use crate::{
    diagnostic::{DiagKind, DiagLevel, Diagnostic},
    ir::{AddressTuple, Either, Immediate, Ir, IrRegister},
    lexer::Cursor,
    span::Span,
    tokens::{Base, Keyword, Register, Token, TokenKind},
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
        let keyword = match self.cursor.advance_token() {
            match_token_kind!(TokenKind::LineComment) => return,
            match_token_kind!(TokenKind::Keyword(keyword)) => keyword,
            token => {
                self.diagnostics
                    .push(unexpected_token_error(token, "Keyword | Comment"));
                return;
            }
        };

        let parse_result = match keyword {
            Keyword::Nor => self.handle_nor(),
            Keyword::Pc => self.handle_pc(),
            Keyword::Load => self.handle_load(),
            Keyword::Store => self.handle_store(),
            Keyword::Set => self.handle_set(),
            Keyword::Label => self.handle_label(),
            Keyword::Link => todo!(),
            // The remaining keywords would better fit in a macro system, but I don't plan on adding one
        };

        match parse_result {
            Ok(ir) => self.ir.push(ir),
            Err(diagnostic) => self.diagnostics.push(diagnostic),
        };

        self.parse_end_of_line()
    }

    pub fn handle_nor(&mut self) -> Result<Ir<'a>, Diagnostic> {
        Ok(Ir::Nor(self.parse_register()?, self.parse_either()?))
    }

    pub fn handle_pc(&mut self) -> Result<Ir<'a>, Diagnostic> {
        Ok(Ir::Pc(self.parse_address_tuple()?))
    }

    pub fn handle_load(&mut self) -> Result<Ir<'a>, Diagnostic> {
        Ok(Ir::Load(self.parse_address_tuple()?))
    }

    pub fn handle_store(&mut self) -> Result<Ir<'a>, Diagnostic> {
        Ok(Ir::Store(self.parse_address_tuple()?))
    }

    pub fn handle_set(&mut self) -> Result<Ir<'a>, Diagnostic> {
        Ok(Ir::Set(self.parse_immediate()?))
    }

    pub fn handle_label(&mut self) -> Result<Ir<'a>, Diagnostic> {
        let identifier = self.parse_identifier()?;
        Ok(Ir::Label(identifier.0, identifier.1))
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
        // TODO This should work for full labels not just pairs of indexed ones
        let first = self.parse_either()?;
        let second = self.parse_either()?;
        Ok(AddressTuple(first, second))
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
            unexpected => Err(unexpected_token_error(
                unexpected,
                "OpenParen | Bang | Numeric | Label",
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

        let numeric = u6_from_str_radix(&self.raw[span.start + prefix_len + 1..span.end], radix)
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
