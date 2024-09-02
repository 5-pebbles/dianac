use arbitrary_int::{u12, u6};
use std::collections::{hash_map::Entry, HashMap, HashSet};
use strum::IntoEnumIterator;

use super::{
    diagnostic::{DiagKind, DiagLevel, Diagnostic},
    ir::{AddressTuple, Conditional, Either, Immediate, Ir, IrRegister},
    span::Span,
};

const MEM_REGISTER: IrRegister = IrRegister::C;

macro_rules! free_register {
    ($($used:ident),*) => {
        {let used = HashSet::from([$($used),*]);

        IrRegister::iter()
            .rev()
            .find(|ir| !used.contains(ir))}
    };
}

fn generate_arithmetic_register_distribution(
    destination: IrRegister,
    source: Either,
    carry: IrRegister,
) -> (IrRegister, IrRegister) {
    if destination == carry {
        let secondary = match source {
            Either::Immediate(_) => free_register!(carry).unwrap(),
            Either::Register(value) if value == carry => free_register!(carry).unwrap(),
            Either::Register(value) => value,
        };
        return (free_register!(secondary, carry).unwrap(), secondary);
    };

    let secondary = free_register!(destination, carry).unwrap();
    (destination, secondary)
}

#[derive(Debug, Clone)]
pub struct IrGenerator<'a> {
    ir: Vec<Ir<'a>>,
    next_address: u12,
    symbol_table: HashMap<&'a str, u12>,
}

impl<'a> IrGenerator<'a> {
    pub fn push(&mut self, value: Ir<'a>) -> &mut Self {
        self.next_address += value.len();
        self.ir.push(value);
        self
    }

    pub fn finalize(self) -> (Vec<Ir<'a>>, HashMap<&'a str, u12>) {
        (self.ir, self.symbol_table)
    }

    // Keywords

    // Bitwise Logic
    pub fn not(&mut self, register: IrRegister) -> &mut Self {
        self.nor(register, Either::Register(register))
    }

    pub fn and(&mut self, register: IrRegister, either: Either<'a>) -> &mut Self {
        let negated_either = match either {
            Either::Register(register) => {
                self.not(register);
                Either::Register(register)
            }
            Either::Immediate(immediate) => Either::Immediate(Immediate::Not(Box::new(immediate))),
        };

        self.not(register).nor(register, negated_either)
    }

    pub fn nand(&mut self, register: IrRegister, either: Either<'a>) -> &mut Self {
        self.and(register, either).not(register)
    }

    pub fn or(&mut self, register: IrRegister, either: Either<'a>) -> &mut Self {
        self.nor(register, either).not(register)
    }

    pub fn nor(&mut self, register: IrRegister, either: Either<'a>) -> &mut Self {
        self.push(Ir::Nor(register, either))
    }

    pub fn xor(&mut self, register: IrRegister, either: Either<'a>) -> &mut Self {
        self.nxor(register, either).not(register)
    }

    pub fn nxor(&mut self, register: IrRegister, either: Either<'a>) -> &mut Self {
        let free_register = if let Either::Register(other_register) = either {
            free_register!(register, other_register).unwrap()
        } else {
            free_register!(register).unwrap()
        };

        self.mov(free_register, Either::Register(register))
            .nor(free_register, either.clone())
            .nor(register, Either::Register(free_register))
            .nor(register, either)
            .nor(register, Either::Register(free_register))
    }

    // Shift and Rotate
    pub fn rol(&mut self, register: IrRegister) -> &mut Self {
        self.lod(AddressTuple(
            Either::Immediate(Immediate::Constant(u6::new(0b111110))),
            Either::Register(register),
        ))
    }

    pub fn ror(&mut self, register: IrRegister) -> &mut Self {
        self.lod(AddressTuple(
            Either::Immediate(Immediate::Constant(u6::new(0b111111))),
            Either::Register(register),
        ))
    }

    pub fn shl(&mut self, register: IrRegister) -> &mut Self {
        self.rol(register).and(
            MEM_REGISTER,
            Either::Immediate(Immediate::Constant(u6::new(0b111110))),
        )
    }

    pub fn shr(&mut self, register: IrRegister) -> &mut Self {
        self.ror(register).and(
            MEM_REGISTER,
            Either::Immediate(Immediate::Constant(u6::new(0b011111))),
        )
    }

    // Arithmetic
    pub fn add(&mut self, register: IrRegister, either: Either<'a>) -> &mut Self {
        // TODO this could be optimized further (mov followed by and should be expanded)
        let carry = MEM_REGISTER;
        let (augend, addend) =
            generate_arithmetic_register_distribution(register, either.clone(), carry);

        self.mov(addend, either)
            .mov(augend, Either::Register(register))
            .mov(carry, Either::Register(register));

        (0..6).into_iter().for_each(|i| {
            if i != 0 {
                self.rol(carry)
                    .mov(addend, Either::Register(augend))
                    .mov(augend, Either::Register(carry));
            }

            self.and(carry, Either::Register(addend))
                .not(addend)
                .nor(augend, Either::Register(addend))
                .nor(augend, Either::Register(carry));
        });

        self.mov(register, Either::Register(augend))
    }

    pub fn sub(&mut self, register: IrRegister, either: Either<'a>) -> &mut Self {
        let carry = MEM_REGISTER;
        let (minuend, subtrahend) =
            generate_arithmetic_register_distribution(register, either.clone(), carry);

        self.mov(subtrahend, either)
            .mov(minuend, Either::Register(register));

        (0..6).into_iter().for_each(|i| {
            if i != 0 {
                self.rol(carry).mov(subtrahend, Either::Register(carry));
            }

            self.nor(
                carry,
                Either::Immediate(Immediate::Constant(u6::new(0b111111))),
            )
            .nor(carry, Either::Register(subtrahend))
            .nor(carry, Either::Register(minuend))
            .not(minuend)
            .nor(minuend, Either::Register(subtrahend))
            .or(minuend, Either::Register(carry));
        });

        self.mov(register, Either::Register(minuend))
    }

    // Memory
    pub fn set(&mut self, immediate: Immediate<'a>) -> &mut Self {
        self.push(Ir::Set(immediate))
    }

    pub fn mov(&mut self, register: IrRegister, either: Either<'a>) -> &mut Self {
        if matches!(either, Either::Register(ref value) if *value == register) {
            return self;
        }

        self.nor(
            register,
            Either::Immediate(Immediate::Constant(u6::new(0b111111))),
        )
        .nor(register, either)
        .not(register)
    }

    pub fn lod(&mut self, address: AddressTuple<'a>) -> &mut Self {
        self.push(Ir::Lod(address))
    }

    pub fn sto(&mut self, address: AddressTuple<'a>) -> &mut Self {
        self.push(Ir::Sto(address))
    }

    // Jump
    pub fn pc(&mut self, address: AddressTuple<'a>) -> &mut Self {
        self.push(Ir::Pc(address))
    }

    pub fn lab(&mut self, label: &'a str, span: Span) -> Result<&mut Self, Diagnostic> {
        match self.symbol_table.entry(label) {
            Entry::Vacant(entry) => {
                entry.insert(self.next_address);
                Ok(self)
            }
            Entry::Occupied(_) => Err(Diagnostic {
                level: DiagLevel::Fatal,
                span,
                kind: DiagKind::DuplicateLabel,
            }),
        }
    }

    pub fn lih(&mut self, condition: Conditional, address: AddressTuple<'a>) -> &mut Self {
        todo!()
    }

    // Miscellaneous
    pub fn hlt(&mut self) -> &mut Self {
        self.push(Ir::Hlt)
    }
}

impl<'a> Default for IrGenerator<'a> {
    fn default() -> Self {
        Self {
            ir: Vec::default(),
            next_address: u12::new(1),
            symbol_table: HashMap::default(),
        }
    }
}
