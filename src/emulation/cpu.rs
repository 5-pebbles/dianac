use arbitrary_int::u6;

use crate::instruction::{Instruction, Operation, Register};

use super::{memory::Memory, program_counter::ProgramCounter};

pub struct Cpu {
    register_a: u6,
    register_b: u6,
    register_c: u6,
    program_counter: ProgramCounter,
    memory: Memory,
}

impl Cpu {
    pub fn new(data: Vec<u6>) -> Self {
        let program_counter = ProgramCounter::default();

        Self {
            register_a: u6::default(),
            register_b: u6::default(),
            register_c: u6::default(),
            memory: Memory::new(program_counter.clone(), data),
            program_counter,
        }
    }

    pub fn consume_until_halt(&mut self) {
        while !self.is_halt() {
            self.consume_instruction();
        }
    }

    fn is_halt(&self) -> bool {
        self.memory.read(self.program_counter.as_tuple()) == u6::new(0b001111)
    }

    fn consume_operand(&mut self, operand: Register) -> u6 {
        match operand {
            Register::A => self.register_a,
            Register::B => self.register_b,
            Register::C => self.register_c,
            Register::Immediate => {
                self.program_counter.increment();
                self.memory.read(self.program_counter.as_tuple())
            }
        }
    }

    fn consume_instruction(&mut self) {
        debug_assert!(!self.is_halt());

        let instruction =
            Instruction::new_with_raw_value(self.memory.read(self.program_counter.as_tuple()));

        if instruction.operation() == Operation::Nor && instruction.one() == Register::Immediate {
            // TODO add special instructions
            unimplemented!()
        }

        let operand_one = self.consume_operand(instruction.one());
        let operand_two = self.consume_operand(instruction.two());
        self.program_counter.increment();

        match instruction.operation() {
            Operation::Nor => {
                let new_value = !(operand_one | operand_two);
                match instruction.one() {
                    Register::A => self.register_a = new_value,
                    Register::B => self.register_b = new_value,
                    Register::C => self.register_c = new_value,
                    _ => unreachable!(),
                }
            }
            Operation::Pc => self.program_counter.set((operand_one, operand_two)),
            Operation::Load => self.register_c = self.memory.read((operand_one, operand_two)),
            Operation::Store => self
                .memory
                .write((operand_one, operand_two), self.register_c),
        }
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "CPU {{
    a: {:0>6b},
    b: {:0>6b},
    c: {:0>6b},
    pc: {:?}
}}",
                self.register_a, self.register_b, self.register_c, self.program_counter,
            )
        } else {
            write!(
                f,
                "CPU {{ a: {:0>6b}, b: {:0>6b}, c: {:0>6b}, pc: {:?} }}",
                self.register_a, self.register_b, self.register_c, self.program_counter,
            )
        }
    }
}
