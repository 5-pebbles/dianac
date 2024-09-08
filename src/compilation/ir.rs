use std::{collections::HashMap, sync::Arc};

use arbitrary_int::{u12, u6};
use strum::EnumIter;

use crate::compilation::{
    diagnostic::{DiagKind, DiagLevel, Diagnostic},
    span::Span,
};

#[derive(Debug, Clone)]
pub enum Ir {
    Nor(IrRegister, Either),
    Pc(AddressTuple),
    Lod(AddressTuple),
    Sto(AddressTuple),
    Set(Immediate),
    Nop,
    Hlt,
}

impl Ir {
    pub fn len(&self) -> u12 {
        match self {
            Self::Nor(_, either) => u12::new(1) + either.len(),
            Self::Pc(address) | Self::Lod(address) | Self::Sto(address) => {
                u12::new(1) + address.len()
            }
            Self::Set(_) => u12::new(1),
            Self::Nop => u12::new(1),
            Self::Hlt => u12::new(1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddressTuple(pub Either, pub Either);

impl AddressTuple {
    pub fn len(&self) -> u12 {
        self.0.len() + self.1.len()
    }
}

#[derive(Debug, Clone)]
pub enum Either {
    Register(IrRegister),
    Immediate(Immediate),
}

impl Either {
    pub fn len(&self) -> u12 {
        u12::new(matches!(self, Self::Immediate(_)) as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum IrRegister {
    A,
    B,
    C,
}

#[derive(Debug, Clone)]
pub struct Conditional {
    pub left: Either,
    pub kind: ConditionalKind,
    pub right: Either,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalKind {
    Eq,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
}

#[derive(Debug, Clone)]
pub enum Immediate {
    Constant(u6),
    LabelP0(Arc<str>, Span),
    LabelP1(Arc<str>, Span),
    Not(Box<Immediate>),
    And(Box<Immediate>, Box<Immediate>),
    Or(Box<Immediate>, Box<Immediate>),
    Add(Box<Immediate>, Box<Immediate>),
    Sub(Box<Immediate>, Box<Immediate>),
    Mul(Box<Immediate>, Box<Immediate>),
    Div(Box<Immediate>, Box<Immediate>),
    Rol(Box<Immediate>, Box<Immediate>),
    Ror(Box<Immediate>, Box<Immediate>),
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

impl Immediate {
    pub fn flatten(&self, symbol_table: &HashMap<Arc<str>, u12>) -> Result<u6, Diagnostic> {
        Ok(match self {
            Immediate::Constant(value) => *value,
            Immediate::LabelP0(value, span) => u12_to_u6(
                *symbol_table
                    .get(value)
                    .ok_or_else(|| undefined_label_error(*span))?
                    >> 6,
            ),
            Immediate::LabelP1(value, span) => u12_to_u6(
                *symbol_table
                    .get(value)
                    .ok_or_else(|| undefined_label_error(*span))?,
            ),
            Immediate::Not(value) => !value.flatten(symbol_table)?,
            Immediate::And(first, second) => {
                first.flatten(symbol_table)? & second.flatten(symbol_table)?
            }
            Immediate::Or(first, second) => {
                first.flatten(symbol_table)? | second.flatten(symbol_table)?
            }
            Immediate::Add(first, second) => {
                first.flatten(symbol_table)? + second.flatten(symbol_table)?
            }
            Immediate::Sub(first, second) => {
                first.flatten(symbol_table)? - second.flatten(symbol_table)?
            }
            Immediate::Mul(first, second) => {
                first.flatten(symbol_table)? * second.flatten(symbol_table)?
            }
            Immediate::Div(first, second) => {
                first.flatten(symbol_table)? / second.flatten(symbol_table)?
            }
            Immediate::Rol(first, second) => first
                .flatten(symbol_table)?
                .rotate_left(second.flatten(symbol_table)?.into()),
            Immediate::Ror(first, second) => first
                .flatten(symbol_table)?
                .rotate_right(second.flatten(symbol_table)?.into()),
        })
    }
}
