use arbitrary_int::u6;
use bitbybit::{bitenum, bitfield};

#[bitfield(u6)]
pub struct Instruction {
    #[bits(4..=5, rw)]
    pub operation: Operation,
    #[bits(2..=3, rw)]
    pub one: Register,
    #[bits(0..=1, rw)]
    pub two: Register,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:06b}", self.raw_value())
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}-{:?}-{:?}",
            self.operation(),
            self.one(),
            self.two()
        )
    }
}

#[derive(Debug, PartialEq)]
#[bitenum(u2, exhaustive = true)]
pub enum Operation {
    Nor = 0b00,
    Pc = 0b01,
    Load = 0b10,
    Store = 0b11,
}

#[derive(Debug, PartialEq)]
#[bitenum(u2, exhaustive = true)]
pub enum Register {
    A = 0b00,
    B = 0b01,
    C = 0b10,
    Immediate = 0b11,
}
