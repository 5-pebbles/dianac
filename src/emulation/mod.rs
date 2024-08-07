use std::{fs::read, path::Path};

use arbitrary_int::u6;

use crate::errors::Error;

use self::cpu::Cpu;

mod cpu;
mod memory;
mod program_counter;

pub fn emulate_impl(data: Vec<u6>) {
    let mut state = Cpu::new(data);
    state.consume_until_halt();
    println!("{:#?}", state);
}

pub fn emulate_file(source: &Path) -> Result<(), Error> {
    let machine_code = read(source)?;
    // TODO This will panic if the first two bits are not zero; add a message for the user
    emulate_impl(machine_code.into_iter().map(|i| u6::new(i)).collect());
    Ok(())
}
