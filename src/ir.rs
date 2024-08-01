use std::collections::HashMap;

use arbitrary_int::{u12, u6};

use crate::{
    diagnostic::{DiagKind, DiagLevel, Diagnostic},
    span::Span,
};

#[derive(Debug, Clone)]
pub enum Ir<'a> {
    Nor(IrRegister, Either<'a>),
    Pc(AddressTuple<'a>),
    Load(AddressTuple<'a>),
    Store(AddressTuple<'a>),
    Set(Immediate<'a>),
    Label(&'a str, Span),
}

impl<'a> Ir<'a> {
    pub fn len(&self) -> u12 {
        match self {
            Self::Nor(_, either) => u12::new(1) + either.len(),
            Self::Pc(address) | Self::Load(address) | Self::Store(address) => {
                u12::new(1) + address.len()
            }
            Self::Set(_) => u12::new(1),
            Self::Label(_, _) => u12::new(0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddressTuple<'a>(pub Either<'a>, pub Either<'a>);

impl<'a> AddressTuple<'a> {
    pub fn len(&self) -> u12 {
        self.0.len() + self.1.len()
    }
}

#[derive(Debug, Clone)]
pub enum Either<'a> {
    Register(IrRegister),
    Immediate(Immediate<'a>),
}

impl<'a> Either<'a> {
    pub fn len(&self) -> u12 {
        u12::new(matches!(self, Self::Immediate(_)) as u16)
    }
}

#[derive(Debug, Clone)]
pub enum IrRegister {
    A,
    B,
    C,
}

#[derive(Debug, Clone)]
pub enum Immediate<'a> {
    Constant(u6),
    LabelP0(&'a str, Span),
    LabelP1(&'a str, Span),
    Not(Box<Immediate<'a>>),
    Or(Box<Immediate<'a>>, Box<Immediate<'a>>),
}

fn u12_to_u6(value: u12) -> u6 {
    u6::new(value.value() as u8 & 0b111111)
}

fn undefined_label_error(span: Span) -> Diagnostic {
    Diagnostic {
        level: DiagLevel::Fatal,
        span,
        kind: DiagKind::UndefinedLabel,
    }
}

impl<'a> Immediate<'a> {
    pub fn flatten(&self, symbol_table: &HashMap<&'a str, u12>) -> Result<u6, Diagnostic> {
        Ok(match self {
            Immediate::Constant(value) => value.clone(),
            Immediate::LabelP0(value, span) => u12_to_u6(
                *symbol_table
                    .get(value)
                    .ok_or_else(|| undefined_label_error(span.clone()))?
                    >> 6,
            ),
            Immediate::LabelP1(value, span) => u12_to_u6(
                *symbol_table
                    .get(value)
                    .ok_or_else(|| undefined_label_error(span.clone()))?,
            ),
            Immediate::Not(value) => !value.flatten(symbol_table)?,
            Immediate::Or(first, second) => {
                first.flatten(symbol_table)? | second.flatten(symbol_table)?
            }
        })
    }
}
