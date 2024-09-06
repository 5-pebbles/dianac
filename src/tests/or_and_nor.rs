use crate::{compilation::compile_to_binary, emulation::InteractiveState};
use arbitrary_int::u6;

#[test]
fn nor_registers() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("NOR A B\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(0b101010);
    state.b = u6::new(0b110011);
    state.consume_until_halt();
    assert_eq!(
        (state.a, state.b, state.c),
        (u6::new(0b000100), u6::new(0b110011), u6::new(0))
    );
}

#[test]
fn nor_immediate() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("NOR A 0b010111\nHLT\nNOR A 0b100101\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();
    assert_eq!(
        (state.a, state.b, state.c),
        (u6::new(0b101000), u6::new(0), u6::new(0))
    );
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!(
        (state.a, state.b, state.c),
        (u6::new(0b010010), u6::new(0), u6::new(0))
    );
}

#[test]
fn or_registers() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("OR A B\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(0b101010);
    state.b = u6::new(0b110011);
    state.consume_until_halt();
    assert_eq!(
        (state.a, state.b, state.c),
        (u6::new(0b111011), u6::new(0b110011), u6::new(0))
    );
}

#[test]
fn or_immediate() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("OR A 0b010111\nHLT\nOR A 0b100101\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();
    assert_eq!(
        (state.a, state.b, state.c),
        (u6::new(0b010111), u6::new(0), u6::new(0))
    );
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!(
        (state.a, state.b, state.c),
        (u6::new(0b110111), u6::new(0), u6::new(0))
    );
}
