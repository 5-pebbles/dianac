use arbitrary_int::u6;

use crate::{compilation::compile_to_binary, emulation::InteractiveState};

#[test]
fn nxor_registers() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("NXOR A B\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(0b101010);
    state.b = u6::new(0b110011);
    state.consume_until_halt();
    assert_eq!((state.a, state.b), (u6::new(0b100110), u6::new(0b110011)));
}

#[test]
fn nxor_immediate() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("NXOR A 0b010111\nHLT\nNXOR A 0b100101\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();
    assert_eq!((state.a, state.b), (u6::new(0b101000), u6::new(0)));
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!((state.a, state.b), (u6::new(0b110010), u6::new(0)));
}

#[test]
fn xor_registers() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("XOR A B\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(0b101010);
    state.b = u6::new(0b110011);
    state.consume_until_halt();
    assert_eq!((state.a, state.b), (u6::new(0b011001), u6::new(0b110011)));
}

#[test]
fn xor_immediate() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("XOR A 0b010111\nHLT\nXOR A 0b100101\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();
    assert_eq!((state.a, state.b), (u6::new(0b010111), u6::new(0)));
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!((state.a, state.b), (u6::new(0b110010), u6::new(0)));
}
