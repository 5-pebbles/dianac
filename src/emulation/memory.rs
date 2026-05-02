use arbitrary_int::u6;

use crate::{emulation::program_counter::ProgramCounter, utils::tuple_as_usize};

const RAM_SIZE: usize = 3902;

pub struct Memory {
    pub ram: [u6; RAM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [u6::default(); RAM_SIZE],
        }
    }

    pub fn store_array(&mut self, offset: usize, machine_code: &[u6]) {
        self.ram[offset..offset + machine_code.len()].copy_from_slice(&machine_code);
    }

    pub fn read(&self, address: (u6, u6), pc: ProgramCounter) -> u6 {
        let as_usize = tuple_as_usize(address);
        let (pc_low, pc_high) = pc.as_tuple();

        match as_usize {
            0x000..=0xF3D => self.ram[as_usize],
            0xF3E => pc_low,
            0xF3F => pc_high,
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
