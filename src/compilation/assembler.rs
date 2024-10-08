use std::{collections::HashMap, sync::Arc};

use arbitrary_int::{u12, u6};

use crate::{
    compilation::{
        diagnostic::Diagnostic,
        ir::{AddressTuple, Either, Immediate, Ir, IrRegister},
    },
    instruction::{Instruction, Operation, Register},
};

// TODO refactor

fn ir_into_register(register: &IrRegister) -> Register {
    match register {
        IrRegister::A => Register::A,
        IrRegister::B => Register::B,
        IrRegister::C => Register::C,
    }
}

fn get_register_or_immediate(
    either: &Either,
    symbol_table: &HashMap<Arc<str>, u12>,
) -> Result<(Register, Option<u6>), Diagnostic> {
    Ok(match either {
        Either::Register(register) => (ir_into_register(register), None),
        Either::Immediate(immediate) => {
            (Register::Immediate, Some(immediate.flatten(symbol_table)?))
        }
    })
}

pub fn assemble<'a>(
    ir: impl IntoIterator<Item = &'a Ir>,
    symbol_table: &HashMap<Arc<str>, u12>,
) -> (Vec<Instruction>, Vec<Diagnostic>) {
    let mut instructions = Vec::new();
    let mut diagnostics = Vec::new();

    ir.into_iter()
        .for_each(|ir| match assemble_ir(ir, &symbol_table) {
            Ok(value) => instructions.extend(value),
            Err(value) => diagnostics.push(value),
        });

    (instructions, diagnostics)
}

fn assemble_ir(
    ir: &Ir,
    symbol_table: &HashMap<Arc<str>, u12>,
) -> Result<Vec<Instruction>, Diagnostic> {
    match ir {
        Ir::Nor(register, either) => handle_nor(register, either, symbol_table),
        Ir::Pc(address) => handle_addressable(Operation::Pc, address, symbol_table),
        Ir::Lod(address) => handle_addressable(Operation::Load, address, symbol_table),
        Ir::Sto(address) => handle_addressable(Operation::Store, address, symbol_table),
        Ir::Set(immediate) => handle_set(immediate, symbol_table),
        Ir::Nop => Ok(handle_nop()),
        Ir::Hlt => Ok(handle_hlt()),
    }
}

macro_rules! if_let_push_some {
    ($instructions:ident, $value:ident) => {
        if let Some(value) = $value {
            $instructions.push(Instruction::new_with_raw_value(value));
        }
    };
}

fn handle_nor(
    register: &IrRegister,
    either: &Either,
    symbol_table: &HashMap<Arc<str>, u12>,
) -> Result<Vec<Instruction>, Diagnostic> {
    let mut instructions = Vec::new();
    let (second_register, immediate) = get_register_or_immediate(either, symbol_table)?;

    instructions.push(
        Instruction::builder()
            .with_operation(Operation::Nor)
            .with_one(ir_into_register(register))
            .with_two(second_register)
            .build(),
    );

    if_let_push_some!(instructions, immediate);

    Ok(instructions)
}

fn handle_addressable(
    operation: Operation,
    address: &AddressTuple,
    symbol_table: &HashMap<Arc<str>, u12>,
) -> Result<Vec<Instruction>, Diagnostic> {
    let mut instructions = Vec::new();

    let (first_register, first_immediate) = get_register_or_immediate(&address.0, symbol_table)?;
    let (second_register, second_immediate) = get_register_or_immediate(&address.1, symbol_table)?;

    instructions.push(
        Instruction::builder()
            .with_operation(operation)
            .with_one(first_register)
            .with_two(second_register)
            .build(),
    );

    if_let_push_some!(instructions, first_immediate);
    if_let_push_some!(instructions, second_immediate);

    Ok(instructions)
}

fn handle_set(
    immediate: &Immediate,
    symbol_table: &HashMap<Arc<str>, u12>,
) -> Result<Vec<Instruction>, Diagnostic> {
    Ok(vec![Instruction::new_with_raw_value(
        immediate.flatten(symbol_table)?,
    )])
}

fn handle_nop() -> Vec<Instruction> {
    vec![Instruction::new_with_raw_value(u6::new(0b001100))]
}

fn handle_hlt() -> Vec<Instruction> {
    vec![Instruction::new_with_raw_value(u6::new(0b001111))]
}
