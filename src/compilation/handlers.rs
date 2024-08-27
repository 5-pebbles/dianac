use arbitrary_int::u6;
use std::collections::HashSet;
use strum::IntoEnumIterator;

use super::{
    ir::{AddressTuple, Conditional, Either, Immediate, Ir, IrRegister},
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
        {let used = HashSet::from([$($used),*]);

        IrRegister::iter()
            .rev()
            .find(|ir| !used.contains(ir))}
    };
}

fn generate_arithmetic_register_distribution(
    base_operand: Either,
    other_operand: Either,
    carry: IrRegister,
) -> (IrRegister, IrRegister) {
    // yum spaghetti
    match base_operand {
        Either::Immediate(_) => {
            let secondary = match other_operand {
                Either::Immediate(_) => free_register!(carry).unwrap(),
                Either::Register(value) if value == carry => free_register!(carry).unwrap(),
                Either::Register(value) => value,
            };
            (free_register!(secondary, carry).unwrap(), secondary)
        }
        Either::Register(register) => {
            if register == carry {
                let secondary = match other_operand {
                    Either::Immediate(_) => free_register!(carry).unwrap(),
                    Either::Register(value) if value == carry => free_register!(carry).unwrap(),
                    Either::Register(value) => value,
                };
                (free_register!(secondary, carry).unwrap(), secondary)
            } else {
                let secondary = free_register!(register, carry).unwrap();
                (register, secondary)
            }
        }
    }
}

pub fn not<'a>(register: IrRegister) -> Vec<Ir<'a>> {
    VecBuilder::new()
        .push(Ir::Nor(register, Either::Register(register)))
        .build()
}

pub fn and(register: IrRegister, either: Either) -> Vec<Ir> {
    let mut builder = VecBuilder::new();

    let either = match either {
        Either::Register(ref other_register) => {
            builder.extend(not(*other_register));
            either
        }
        Either::Immediate(immediate) => Either::Immediate(Immediate::Not(Box::new(immediate))),
    };

    builder
        .extend(not(register))
        .push(Ir::Nor(register, either))
        .build()
}

pub fn nand(register: IrRegister, either: Either) -> Vec<Ir> {
    VecBuilder::new()
        .extend(and(register, either))
        .extend(not(register))
        .build()
}

pub fn or(register: IrRegister, either: Either) -> Vec<Ir> {
    VecBuilder::new()
        .extend(nor(register, either))
        .extend(not(register))
        .build()
}

pub fn nor(register: IrRegister, either: Either) -> Vec<Ir> {
    VecBuilder::new().push(Ir::Nor(register, either)).build()
}

pub fn xor(register: IrRegister, either: Either) -> Vec<Ir> {
    VecBuilder::new()
        .extend(nxor(register, either))
        .extend(not(register))
        .build()
}

pub fn nxor(register: IrRegister, either: Either) -> Vec<Ir> {
    let free_register = if let Either::Register(other_register) = either {
        free_register!(register, other_register).unwrap()
    } else {
        free_register!(register).unwrap()
    };

    VecBuilder::new()
        .extend(mov(free_register, Either::Register(register)))
        .push(Ir::Nor(free_register, either.clone()))
        .push(Ir::Nor(register, Either::Register(free_register)))
        .push(Ir::Nor(free_register, either))
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

pub fn add(register: IrRegister, either: Either) -> Vec<Ir> {
    // TODO this could be optimized further (mov followed by and should be expanded)
    let mut builder = VecBuilder::new();

    // this is the most efficient allocation of registers
    let carry = IrRegister::C;
    let (primary, secondary) = generate_arithmetic_register_distribution(
        Either::Register(register),
        either.clone(),
        carry,
    );

    // mov will optimize if source == destination
    builder.extend(mov(secondary, either));
    builder.extend(mov(primary, Either::Register(register)));
    builder.extend(mov(carry, Either::Register(register)));

    (0..6).into_iter().for_each(|i| {
        if i != 0 {
            // carry = rol(carry) this is why carry is always C
            builder.extend(rol(Either::Register(carry)));
            // secondary = carry
            builder.extend(mov(secondary, Either::Register(carry)));
            builder.extend(mov(carry, Either::Register(primary)));
        };
        // carry = and(primary, secondary)
        builder
            .extend(and(carry, Either::Register(secondary)))
            .extend(not(secondary));
        // primary = xor(primary, secondary)
        builder
            .push(Ir::Nor(primary, Either::Register(secondary)))
            .push(Ir::Nor(primary, Either::Register(carry)));
    });

    builder.extend(mov(register, Either::Register(primary)));

    builder.build()
}

pub fn sub(register: IrRegister, either: Either) -> Vec<Ir> {
    // TODO this could be optimized further
    let mut builder = VecBuilder::new();

    // this is the most efficient allocation of registers
    let carry = IrRegister::C;
    let (minuend, subtrahend) = generate_arithmetic_register_distribution(
        Either::Register(register),
        either.clone(),
        carry,
    );

    // mov will optimize if source == destination
    builder.extend(mov(subtrahend, either));
    builder.extend(mov(minuend, Either::Register(register)));

    (0..6).into_iter().for_each(|i| {
        if i != 0 {
            builder
                .extend(rol(Either::Register(carry)))
                .extend(mov(subtrahend, Either::Register(carry)));
        }

        builder
            .push(Ir::Nor(
                carry,
                Either::Immediate(Immediate::Constant(u6::new(0b111111))),
            ))
            .push(Ir::Nor(carry, Either::Register(subtrahend)))
            .push(Ir::Nor(carry, Either::Register(minuend)));

        builder
            .extend(not(minuend))
            .push(Ir::Nor(minuend, Either::Register(subtrahend)));

        builder.extend(or(minuend, Either::Register(carry)));
    });

    builder.extend(mov(register, Either::Register(minuend)));

    builder.build()
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
            register,
            Either::Immediate(Immediate::Constant(u6::new(0b111111))),
        ))
        .push(Ir::Nor(register, either))
        .extend(not(register))
        .build()
}

pub fn lod(address: AddressTuple) -> Vec<Ir> {
    VecBuilder::new().push(Ir::Load(address)).build()
}

pub fn sto(address: AddressTuple) -> Vec<Ir> {
    VecBuilder::new().push(Ir::Store(address)).build()
}

pub fn pc(address: AddressTuple) -> Vec<Ir> {
    VecBuilder::new().push(Ir::Pc(address)).build()
}

pub fn lab(label: &str, span: Span) -> Vec<Ir> {
    VecBuilder::new().push(Ir::Label(label, span)).build()
}

pub fn lih<'a>(condition: Conditional, address: AddressTuple) -> Vec<Ir<'a>> {
    Vec::new()
}

pub fn hlt<'a>() -> Vec<Ir<'a>> {
    VecBuilder::new()
        .push(Ir::Set(Immediate::Constant(u6::new(0b001111))))
        .build()
}
