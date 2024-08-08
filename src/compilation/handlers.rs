use arbitrary_int::u6;
use std::collections::HashSet;
use strum::IntoEnumIterator;

use crate::compilation::{
    ir::{AddressTuple, Either, Immediate, Ir, IrRegister},
    span::Span,
};

/// A builder struct for creating vectors with a fluent interface.
///
/// # Warning
///
/// The `build` method resets the internal vector. Calling `build` multiple times
/// will result in empty vectors after the first call.
///
/// # Examples
///
/// ```
/// let mut builder = VecBuilder::new();
/// let vec = builder.push(1).push(2).extend(vec![3, 4, 5]).build();
/// assert_eq!(vec, vec![1, 2, 3, 4, 5]);
///
/// // Subsequent calls to build will return an empty vector
/// let empty_vec = builder.build();
/// assert_eq!(empty_vec, Vec::<i32>::new());
/// ```
struct VecBuilder<T>(Vec<T>);

impl<T> VecBuilder<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, value: T) -> &mut Self {
        self.0.push(value);
        self
    }

    pub fn extend(&mut self, value: Vec<T>) -> &mut Self {
        self.0.extend(value);
        self
    }

    pub fn build(&mut self) -> Vec<T> {
        std::mem::take(&mut self.0)
    }
}

macro_rules! free_register {
    ($($used:ident),*) => {
        {let used = HashSet::from([$($used.clone()),*]);

        IrRegister::iter()
            .rev()
            .find(|ir| !used.contains(ir))
            .unwrap()}
    };
}

pub fn not<'a>(register: IrRegister) -> Vec<Ir<'a>> {
    VecBuilder::new()
        .push(Ir::Nor(register.clone(), Either::Register(register)))
        .build()
}

pub fn and(register: IrRegister, either: Either) -> Vec<Ir> {
    let mut builder = VecBuilder::new();

    let either = match either {
        Either::Register(ref other_register) => {
            builder.extend(not(other_register.clone()));
            either
        }
        Either::Immediate(immediate) => Either::Immediate(Immediate::Not(Box::new(immediate))),
    };

    builder
        .extend(not(register.clone()))
        .push(Ir::Nor(register, either))
        .build()
}

pub fn nand(register: IrRegister, either: Either) -> Vec<Ir> {
    VecBuilder::new()
        .extend(and(register.clone(), either))
        .extend(not(register))
        .build()
}

pub fn or(register: IrRegister, either: Either) -> Vec<Ir> {
    VecBuilder::new()
        .extend(nor(register.clone(), either))
        .extend(not(register))
        .build()
}

pub fn nor(register: IrRegister, either: Either) -> Vec<Ir> {
    VecBuilder::new().push(Ir::Nor(register, either)).build()
}

pub fn xor(register: IrRegister, either: Either) -> Vec<Ir> {
    VecBuilder::new()
        .extend(nxor(register.clone(), either))
        .extend(not(register))
        .build()
}

pub fn nxor(register: IrRegister, either: Either) -> Vec<Ir> {
    let free_register = if let Either::Register(other_register) = &either {
        free_register!(register, other_register)
    } else {
        free_register!(register)
    };

    VecBuilder::new()
        .extend(mov(
            free_register.clone(),
            Either::Register(register.clone()),
        ))
        .push(Ir::Nor(free_register.clone(), either.clone()))
        .push(Ir::Nor(
            register.clone(),
            Either::Register(free_register.clone()),
        ))
        .push(Ir::Nor(free_register.clone(), either))
        .push(Ir::Nor(register, Either::Register(free_register)))
        .build()
}

pub fn rol(either: Either) -> Vec<Ir> {
    VecBuilder::new()
        .push(Ir::Load(AddressTuple(
            Either::Immediate(Immediate::Constant(u6::new(0b111110))),
            either,
        )))
        .build()
}

pub fn ror(either: Either) -> Vec<Ir> {
    VecBuilder::new()
        .push(Ir::Load(AddressTuple(
            Either::Immediate(Immediate::Constant(u6::new(0b111111))),
            either,
        )))
        .build()
}

pub fn shl(either: Either) -> Vec<Ir> {
    VecBuilder::new()
        .extend(rol(either))
        .extend(and(
            IrRegister::C,
            Either::Immediate(Immediate::Constant(u6::new(0b111110))),
        ))
        .build()
}

pub fn shr(either: Either) -> Vec<Ir> {
    VecBuilder::new()
        .extend(ror(either))
        .extend(and(
            IrRegister::C,
            Either::Immediate(Immediate::Constant(u6::new(0b011111))),
        ))
        .build()
}

pub fn set(immediate: Immediate) -> Vec<Ir> {
    VecBuilder::new().push(Ir::Set(immediate)).build()
}

pub fn mov(register: IrRegister, either: Either) -> Vec<Ir> {
    if matches!(either, Either::Register(ref value) if *value == register) {
        return Vec::new();
    }

    VecBuilder::new()
        .push(Ir::Nor(
            register.clone(),
            Either::Immediate(Immediate::Constant(u6::new(0b111111))),
        ))
        .push(Ir::Nor(register.clone(), either))
        .extend(not(register))
        .build()
}

pub fn lod(address: AddressTuple) -> Vec<Ir> {
    VecBuilder::new().push(Ir::Load(address)).build()
}

pub fn sto(address: AddressTuple) -> Vec<Ir> {
    VecBuilder::new().push(Ir::Store(address)).build()
}

pub fn lab(label: &str, span: Span) -> Vec<Ir> {
    VecBuilder::new().push(Ir::Label(label, span)).build()
}

pub fn pc(address: AddressTuple) -> Vec<Ir> {
    VecBuilder::new().push(Ir::Pc(address)).build()
}
