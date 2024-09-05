use arbitrary_int::u6;

use crate::instruction::{Instruction, Operation, Register};

use super::{memory::Memory, program_counter::ProgramCounter};

pub struct InteractiveState {
    pub a: u6,
    pub b: u6,
    pub c: u6,
    pub memory: Memory,
    pub program_counter: ProgramCounter,
}

impl InteractiveState {
    pub fn new() -> Self {
        // cloning the program_counter creates a reference to the same data
        let program_counter = ProgramCounter::default();

        Self {
            a: u6::default(),
            b: u6::default(),
            c: u6::default(),
            memory: Memory::new(program_counter.clone()),
            program_counter,
        }
    }

    pub fn consume_until_next_halt(&mut self) {
        self.consume_instruction();
        while !self.is_halt() {
            self.consume_instruction();
        }
    }

    pub fn is_halt(&self) -> bool {
        self.memory.read(self.program_counter.as_tuple()) == u6::new(0b001111)
    }

    fn consume_operand(&mut self, operand: Register) -> u6 {
        match operand {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::Immediate => {
                self.program_counter.increment();
                self.memory.read(self.program_counter.as_tuple())
            }
        }
    }

    pub fn consume_instruction(&mut self) {
        let raw_value = self.memory.read(self.program_counter.as_tuple());

        // TODO add special instructions
        match raw_value.value() {
            0b001100 /* Nop */ => {
                self.program_counter.increment();
                return;
            }
            0b001101 => unimplemented!(),
            0b001110 => unimplemented!(),
            0b001111 /* Hlt */ => {
                self.program_counter.increment();
                return;
            },
            _ => (),
        }

        let instruction = Instruction::new_with_raw_value(raw_value);

        let operand_one = self.consume_operand(instruction.one());
        let operand_two = self.consume_operand(instruction.two());
        self.program_counter.increment();

        match instruction.operation() {
            Operation::Nor => {
                let new_value = !(operand_one | operand_two);
                match instruction.one() {
                    Register::A => self.a = new_value,
                    Register::B => self.b = new_value,
                    Register::C => self.c = new_value,
                    _ => unreachable!(),
                }
            }
            Operation::Pc => self.program_counter.set((operand_one, operand_two)),
            Operation::Load => self.c = self.memory.read((operand_one, operand_two)),
            Operation::Store => self.memory.write((operand_one, operand_two), self.c),
        }
    }
}
