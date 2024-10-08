use arbitrary_int::{u12, u6};
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    sync::Arc,
};
use strum::IntoEnumIterator;

use super::{
    diagnostic::{DiagKind, DiagLevel, Diagnostic},
    ir::{AddressTuple, Conditional, ConditionalKind, Either, Immediate, Ir, IrRegister},
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

pub fn unique_label() -> Arc<str> {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let label = format!(
        "#{}",
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    );
    Arc::from(label)
}

#[derive(Debug, Clone)]
pub struct IrGenerator {
    ir: Vec<Ir>,
    next_address: u12,
    symbol_table: HashMap<Arc<str>, u12>,
}

impl IrGenerator {
    pub fn new(offset: u12) -> Self {
        Self {
            ir: Vec::default(),
            next_address: offset,
            symbol_table: HashMap::default(),
        }
    }

    pub fn push(&mut self, value: Ir) -> &mut Self {
        self.next_address += value.len();
        self.ir.push(value);
        self
    }

    pub fn finalize(self) -> (Vec<Ir>, HashMap<Arc<str>, u12>) {
        (self.ir, self.symbol_table)
    }

    // Keywords

    // Bitwise Logic
    pub fn not(&mut self, register: IrRegister) -> &mut Self {
        self.nor(register, Either::Register(register))
    }

    pub fn and(&mut self, register: IrRegister, either: Either) -> &mut Self {
        let negated_either = match either {
            Either::Register(register) => {
                self.not(register);
                Either::Register(register)
            }
            Either::Immediate(immediate) => Either::Immediate(Immediate::Not(Box::new(immediate))),
        };

        self.not(register).nor(register, negated_either)
    }

    pub fn nand(&mut self, register: IrRegister, either: Either) -> &mut Self {
        self.and(register, either).not(register)
    }

    pub fn or(&mut self, register: IrRegister, either: Either) -> &mut Self {
        self.nor(register, either).not(register)
    }

    pub fn nor(&mut self, register: IrRegister, either: Either) -> &mut Self {
        self.push(Ir::Nor(register, either))
    }

    pub fn xor(&mut self, register: IrRegister, either: Either) -> &mut Self {
        self.nxor(register, either).not(register)
    }

    pub fn nxor(&mut self, register: IrRegister, either: Either) -> &mut Self {
        let free_register = if let Either::Register(other_register) = either {
            free_register!(register, other_register).unwrap()
        } else {
            free_register!(register).unwrap()
        };

        self.mov(free_register, Either::Register(register))
            .nor(free_register, either.clone())
            .nor(register, Either::Register(free_register))
            .nor(free_register, either)
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
    pub fn add(&mut self, register: IrRegister, either: Either) -> &mut Self {
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

    pub fn sub(&mut self, register: IrRegister, either: Either) -> &mut Self {
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
    pub fn set(&mut self, immediate: Immediate) -> &mut Self {
        self.push(Ir::Set(immediate))
    }

    pub fn zero(&mut self, register: IrRegister) -> &mut Self {
        self.nor(
            register,
            Either::Immediate(Immediate::Constant(u6::new(0b111111))),
        )
    }

    pub fn mov(&mut self, register: IrRegister, either: Either) -> &mut Self {
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

    pub fn lod(&mut self, address: AddressTuple) -> &mut Self {
        self.push(Ir::Lod(address))
    }

    pub fn sto(&mut self, address: AddressTuple) -> &mut Self {
        self.push(Ir::Sto(address))
    }

    // Jump
    pub fn pc(&mut self, address: AddressTuple) -> &mut Self {
        self.push(Ir::Pc(address))
    }

    pub fn lab(&mut self, label: Arc<str>, span: Span) -> Result<&mut Self, Diagnostic> {
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

    pub fn lih(&mut self, condition: Conditional, address: AddressTuple) -> &mut Self {
        // This code was once absolute dog shit; it's fixed now, but I am leaving the emoticons: ┗(▀̿ĺ̯▀̿ ̿)┓  ●~*
        match condition.kind {
            ConditionalKind::Eq | ConditionalKind::NotEq => {
                if matches!(condition.left, Either::Register(left) if left == MEM_REGISTER) {
                    self.xor(MEM_REGISTER, condition.right)
                } else {
                    self.mov(MEM_REGISTER, condition.right)
                        .xor(MEM_REGISTER, condition.left)
                };
            }
            ConditionalKind::GreaterEq
            | ConditionalKind::Less
            | ConditionalKind::LessEq
            | ConditionalKind::Greater => {
                let (left, right) = if matches!(
                    condition.kind,
                    ConditionalKind::LessEq | ConditionalKind::Greater
                ) {
                    (condition.right, condition.left)
                } else {
                    (condition.left, condition.right)
                };

                let helper = match (left, right.clone()) {
                    (Either::Register(left_reg), right) if left_reg != MEM_REGISTER => {
                        self.mov(MEM_REGISTER, right);
                        left_reg
                    }
                    (left, Either::Register(right_reg)) => {
                        let free_register = free_register!(right_reg, MEM_REGISTER).unwrap();
                        self.mov(free_register, left).mov(MEM_REGISTER, right);
                        free_register
                    }
                    (left, right) => {
                        let free_register = free_register!(MEM_REGISTER).unwrap();
                        self.mov(free_register, left).mov(MEM_REGISTER, right);
                        free_register
                    }
                };
                // The following operation is (helper <= MEM_REGISTER):

                let other_helper = free_register!(helper, MEM_REGISTER).unwrap();
                (0..6).into_iter().for_each(|i| {
                    self.mov(other_helper, Either::Register(MEM_REGISTER));

                    self.nor(MEM_REGISTER, Either::Register(helper))
                        .nor(MEM_REGISTER, Either::Register(helper));

                    self.nor(helper, Either::Register(other_helper))
                        .nor(helper, Either::Register(other_helper));

                    // This simulates shifting helper in place:
                    // 1. rol(MEM_REGISTER) is the same as ror(helper)
                    // 2. Next we need to mask it like shift would from right to left.
                    //    This can be done with the inverse of [1, 2, 4, 8, 16, 32] or 2.pow(i)
                    self.rol(MEM_REGISTER).and(
                        helper,
                        Either::Immediate(Immediate::Constant(!u6::new(2_u8.pow(i)))),
                    );
                });
            }
        }

        // If the value in MEM_REGISTER is zero then jump.

        let helper = free_register!(MEM_REGISTER).unwrap();
        self.zero(helper);
        // Distribute the value until we have [0b000000 | 0b111111]
        (0..6).into_iter().for_each(|_| {
            self.or(helper, Either::Register(MEM_REGISTER))
                .ror(MEM_REGISTER);
        });

        // Flip if we are using one of the negated conditions.
        if condition.kind == ConditionalKind::NotEq
            || condition.kind == ConditionalKind::Less
            || condition.kind == ConditionalKind::Greater
        {
            self.not(helper);
        }

        // Mask into [0 | 3]
        self.and(
            helper,
            Either::Immediate(Immediate::Constant(u6::new(0b000011))),
        );

        // Add that to our label and jump to the new address.
        let label = unique_label();
        self.add(
            helper,
            Either::Immediate(Immediate::LabelP1(label.clone(), Span::new(0, 0))),
        )
        .pc(AddressTuple(
            Either::Immediate(Immediate::LabelP0(label.clone(), Span::new(0, 0))),
            Either::Register(helper),
        ));

        // Adding a six bit tuple is complicated and expensive.
        // Solution: skip the 6-bit overflow.
        while self.next_address & u12::new(0b111111) > u12::new(0b111111 - 3) {
            self.nop();
        }

        // If we added three to this label we will skip the last instruction otherwise we will jump to the target address
        self.lab(label, Span::new(0, 0)).unwrap().pc(address)
    }

    // Miscellaneous
    pub fn nop(&mut self) -> &mut Self {
        self.push(Ir::Nop)
    }

    pub fn hlt(&mut self) -> &mut Self {
        self.push(Ir::Hlt)
    }
}

impl Default for IrGenerator {
    fn default() -> Self {
        Self::new(u12::default())
    }
}
