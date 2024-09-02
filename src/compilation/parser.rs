use std::{collections::HashMap, num::IntErrorKind};

use arbitrary_int::{u12, u6};

use crate::{
    character_encoding::encode_character,
    compilation::{
        diagnostic::{DiagKind, DiagLevel, Diagnostic},
        generator::IrGenerator,
        ir::{AddressTuple, Conditional, ConditionalKind, Either, Immediate, Ir, IrRegister},
        lexer::Cursor,
        span::Span,
        tokens::{Base, Keyword, Register, Token, TokenKind},
    },
};

macro_rules! token_kind {
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

pub struct ParseResult<'a> {
    pub ir: Vec<Ir<'a>>,
    pub symbol_table: HashMap<&'a str, u12>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    cursor: Cursor<'a>,
    raw: &'a str,
    ir: IrGenerator<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse(mut self) -> ParseResult<'a> {
        let mut diagnostics = Vec::new();
        while !self.cursor.is_eof() {
            self.advance_ir().unwrap_or_else(|e| {
                diagnostics.push(e);
            });
        }

        let (ir, symbol_table) = self.ir.finalize();

        ParseResult {
            ir,
            symbol_table,
            diagnostics,
        }
    }

    pub fn advance_ir(&mut self) -> Result<(), Diagnostic> {
        // TODO move to next line on error
        let keyword = match self.cursor.advance_token() {
            token_kind!(TokenKind::NewLine) => return Ok(()),
            token_kind!(TokenKind::LineComment) => {
                self.parse_end_of_line(false)?;
                return Ok(());
            }
            token_kind!(TokenKind::Keyword(keyword)) => keyword,
            token => {
                return Err(unexpected_token_error(token, "Keyword | Comment | NewLine"));
            }
        };

        match keyword {
            // Bitwise Logic
            Keyword::Not => {
                let register = self.parse_register()?;
                self.ir.not(register);
            }
            Keyword::And => {
                let register = self.parse_register()?;
                let either = self.parse_either()?;
                self.ir.and(register, either);
            }
            Keyword::Nand => {
                let register = self.parse_register()?;
                let either = self.parse_either()?;
                self.ir.nand(register, either);
            }
            Keyword::Or => {
                let register = self.parse_register()?;
                let either = self.parse_either()?;
                self.ir.or(register, either);
            }
            Keyword::Nor => {
                let register = self.parse_register()?;
                let either = self.parse_either()?;
                self.ir.nor(register, either);
            }
            Keyword::Xor => {
                let register = self.parse_register()?;
                let either = self.parse_either()?;
                self.ir.xor(register, either);
            }
            Keyword::Nxor => {
                let register = self.parse_register()?;
                let either = self.parse_either()?;
                self.ir.nxor(register, either);
            }
            // Shift and Rotate
            Keyword::Rol => {
                let register = self.parse_register()?;
                self.ir.rol(register);
            }
            Keyword::Ror => {
                let register = self.parse_register()?;
                self.ir.ror(register);
            }
            Keyword::Shl => {
                let register = self.parse_register()?;
                self.ir.shl(register);
            }
            Keyword::Shr => {
                let register = self.parse_register()?;
                self.ir.shr(register);
            }
            // Arithmetic
            Keyword::Add => {
                let register = self.parse_register()?;
                let either = self.parse_either()?;
                self.ir.add(register, either);
            }
            Keyword::Sub => {
                let register = self.parse_register()?;
                let either = self.parse_either()?;
                self.ir.sub(register, either);
            }
            // Memory
            Keyword::Set => {
                let immediate = self.parse_immediate()?;
                self.ir.set(immediate);
            }
            Keyword::Mov => {
                let register = self.parse_register()?;
                let either = self.parse_either()?;
                self.ir.mov(register, either);
            }
            Keyword::Lod => {
                let address = self.parse_address_tuple()?;
                self.ir.lod(address);
            }
            Keyword::Sto => {
                let address = self.parse_address_tuple()?;
                self.ir.sto(address);
            }
            // Jump
            Keyword::Pc => {
                let address = self.parse_address_tuple()?;
                self.ir.pc(address);
            }
            Keyword::Lab => {
                let (label, span) = self.parse_identifier()?;
                self.ir.lab(label, span)?;
            }
            Keyword::Lih => {
                // this code should work fine and its safe... but it don't...
                // let conditional = self.parse_conditional()?;
                // let address_tuple = self.parse_address_tuple()?;
                // self.ir.lih(conditional, address_tuple);
                let conditional: Conditional =
                    unsafe { core::mem::transmute(self.parse_conditional()?) };
                let address_tuple = self.parse_address_tuple()?;
                self.ir.lih(conditional, address_tuple);
            }
            // Miscellaneous
            Keyword::HLT => {
                self.ir.hlt();
            }
        };

        self.parse_end_of_line(true)?;

        Ok(())
    }

    pub fn parse_identifier(&mut self) -> Result<(&'a str, Span), Diagnostic> {
        match self.cursor.advance_token() {
            token @ token_kind!(TokenKind::Identifier) => {
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
                Either::Immediate(Immediate::LabelP0(label, span)),
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
            token_kind!(TokenKind::Register(register)) => Ok(match register {
                Register::A => IrRegister::A,
                Register::B => IrRegister::B,
                Register::C => IrRegister::C,
            }),
            unexpected => Err(unexpected_token_error(unexpected, "Register")),
        }
    }

    pub fn parse_conditional(&mut self) -> Result<Conditional, Diagnostic> {
        match self.cursor.advance_token() {
            token_kind!(TokenKind::OpenBracket) => (),
            unexpected => return Err(unexpected_token_error(unexpected, "OpenBracket")),
        }

        let first = self.parse_either()?;
        let condition = match self.cursor.advance_token() {
            token_kind!(TokenKind::Eq) => match self.cursor.advance_token() {
                token_kind!(TokenKind::Eq) => ConditionalKind::Eq,
                unexpected => return Err(unexpected_token_error(unexpected, "Eq")),
            },
            token_kind!(TokenKind::Not) => match self.cursor.advance_token() {
                token_kind!(TokenKind::Eq) => ConditionalKind::NotEq,
                unexpected => return Err(unexpected_token_error(unexpected, "Eq")),
            },
            token_kind!(TokenKind::Greater) => match self.cursor.clone().advance_token() {
                token_kind!(TokenKind::Eq) => {
                    self.cursor.advance_token();
                    ConditionalKind::GreaterEq
                }
                _ => ConditionalKind::Greater,
            },
            token_kind!(TokenKind::Less) => match self.cursor.clone().advance_token() {
                token_kind!(TokenKind::Eq) => {
                    self.cursor.advance_token();
                    ConditionalKind::LessEq
                }
                _ => ConditionalKind::Less,
            },
            unexpected => {
                return Err(unexpected_token_error(
                    unexpected,
                    "Eq | Not | Greater | Less",
                ))
            }
        };
        let last = self.parse_either()?;

        match self.cursor.advance_token() {
            token_kind!(TokenKind::CloseBracket) => Ok(Conditional(first, condition, last)),
            unexpected => Err(unexpected_token_error(unexpected, "CloseBracket")),
        }
    }

    pub fn parse_immediate(&mut self) -> Result<Immediate<'a>, Diagnostic> {
        match self.cursor.advance_token() {
            token_kind!(TokenKind::OpenParen) => {
                let immediate = self.parse_immediate()?;
                let block = self.parse_block(immediate)?;
                match self.cursor.advance_token() {
                    token_kind!(TokenKind::CloseParen) => Ok(block),
                    unexpected => Err(unexpected_token_error(unexpected, "CloseParen")),
                }
            }
            token_kind!(TokenKind::Not) => Ok(Immediate::Not(Box::new(self.parse_immediate()?))),
            token @ token_kind!(TokenKind::Identifier) => self.parse_label(token),
            ref token @ token_kind!(TokenKind::Numeric { ref base, ref prefix_len }) => {
                self.parse_numeric(token.span, base, prefix_len)
            }
            ref token @ token_kind!(TokenKind::Character { ref terminated }) => {
                self.parse_character(token.span, &terminated)
            }
            unexpected => Err(unexpected_token_error(
                unexpected,
                "OpenParen | Bang | Numeric | Label | Character",
            )),
        }
    }

    fn parse_block(&mut self, immediate: Immediate<'a>) -> Result<Immediate<'a>, Diagnostic> {
        let mut clone = self.cursor.clone();
        let operator_builder = match clone.advance_token().kind {
            TokenKind::And => |first, second| Immediate::And(first, second),
            TokenKind::Or => |first, second| Immediate::Or(first, second),
            TokenKind::Add => |first, second| Immediate::Add(first, second),
            TokenKind::Sub => |first, second| Immediate::Sub(first, second),
            TokenKind::Mul => |first, second| Immediate::Mul(first, second),
            TokenKind::Div => |first, second| Immediate::Div(first, second),
            TokenKind::Less if clone.advance_token().kind == TokenKind::Less => {
                self.cursor.advance_token();
                |first, second| Immediate::Rol(first, second)
            }
            TokenKind::Greater if clone.advance_token().kind == TokenKind::Greater => {
                self.cursor.advance_token();
                |first, second| Immediate::Ror(first, second)
            }
            _ => return Ok(immediate),
        };

        self.cursor.advance_token();

        let next = self.parse_immediate()?;

        Ok(self.parse_block(operator_builder(Box::new(immediate), Box::new(next)))?)
    }

    fn parse_numeric(
        &self,
        span: Span,
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
                span,
                kind: DiagKind::ParseImmediate(e),
            })?;

        Ok(Immediate::Constant(numeric))
    }

    fn parse_label(&mut self, first: Token) -> Result<Immediate<'a>, Diagnostic> {
        match self.cursor.advance_token() {
            token_kind!(TokenKind::Colon) => (),
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
        span: Span,
        terminated: &bool,
    ) -> Result<Immediate<'a>, Diagnostic> {
        if !terminated {
            return Err(Diagnostic {
                level: DiagLevel::Fatal,
                span,
                kind: DiagKind::IncompleteCharacter,
            });
        }

        let character = self.raw[span.as_range()].chars().nth(1).unwrap();

        encode_character(&character)
            .map(|numeric| Immediate::Constant(*numeric))
            .ok_or(Diagnostic {
                level: DiagLevel::Fatal,
                span,
                kind: DiagKind::UnsupportedCharacter(character),
            })
    }

    pub fn parse_end_of_line(&mut self, allow_trailing_comment: bool) -> Result<(), Diagnostic> {
        match self.cursor.advance_token() {
            token_kind!(TokenKind::NewLine | TokenKind::Eof) => Ok(()),
            token_kind!(TokenKind::LineComment) if allow_trailing_comment => {
                self.parse_end_of_line(false)
            }
            token => Err(unexpected_token_error(
                token,
                if allow_trailing_comment {
                    "NewLine | Eof | Comment"
                } else {
                    "NewLine | Eof"
                },
            )),
        }
    }
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            cursor: Cursor::from(value),
            raw: value,
            ir: IrGenerator::default(),
        }
    }
}
