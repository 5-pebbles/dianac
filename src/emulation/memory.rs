use arbitrary_int::u6;

use crate::utils::tuple_as_usize;

use super::program_counter::ProgramCounter;

const RAM_SIZE: usize = 3902;

pub struct Memory {
    pub program_counter: ProgramCounter,
    pub ram: [u6; RAM_SIZE],
}

impl Memory {
    pub fn new(program_counter: ProgramCounter) -> Self {
        Self {
            program_counter,
            ram: [u6::default(); RAM_SIZE],
        }
    }

    pub fn store_array(&mut self, offset: usize, machine_code: &[u6]) {
        self.ram[offset..offset + machine_code.len()].copy_from_slice(&machine_code);
    }

    pub fn read(&self, address: (u6, u6)) -> u6 {
        let as_usize = tuple_as_usize(address);

        match as_usize {
            0x000..=0xF3D => self.ram[as_usize],
            0xF3E => self.program_counter.as_tuple().0,
            0xF3F => self.program_counter.as_tuple().1,
            0xF80..=0xFBF => address.1.rotate_left(1),
            0xFC0..=0xFFF => address.1.rotate_right(1),
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: (u6, u6), value: u6) {
        let as_usize = tuple_as_usize(address);

        match as_usize {
            0x000..=0xF3D => self.ram[as_usize] = value,
            // TODO I don't know what to use this for, but I am not letting this many addresses go to waste
            0xF3E..=0xFFF => todo!(),
            _ => unreachable!(),
        }
    }
}
