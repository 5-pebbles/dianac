use arbitrary_int::u6;

use crate::{compilation::compile_to_binary, emulation::InteractiveState};

#[test]
fn rol() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("ROL A\nHLT\nROL B\nHLT\nROL C\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(0b101101);
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b011011));
    state.c = u6::new(0);
    state.b = u6::new(0b101101);
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b011011));
    state.c = u6::new(0b101101);
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b011011));
}

#[test]
fn ror() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("ROR A\nHLT\nROR B\nHLT\nROR C\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(0b101101);
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b110110));
    state.c = u6::new(0);
    state.b = u6::new(0b101101);
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b110110));
    state.c = u6::new(0b101101);
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b110110));
}

#[test]
fn shl() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("SHL A\nHLT\nSHL B\nHLT\nSHL C\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(0b101101);
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b011010));
    state.c = u6::new(0);
    state.b = u6::new(0b101101);
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b011010));
    state.c = u6::new(0b101101);
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b011010));
}

#[test]
fn shr() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("SHR A\nHLT\nSHR B\nHLT\nSHR C\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(0b101101);
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b010110));
    state.c = u6::new(0);
    state.b = u6::new(0b101101);
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b010110));
    state.c = u6::new(0b101101);
    state.consume_instruction();
    state.consume_until_halt();
    assert_eq!(state.c, u6::new(0b010110));
}
