use arbitrary_int::{u12, u6};

use crate::{compilation::compile_to_binary, emulation::InteractiveState, utils::tuple_as_u12};

#[test]
fn jump_not_taken_when_a_is_zero() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("LIH [A == 1] TEST\nHLT\nLAB TEST\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();
    assert_eq!(
        Some(&(tuple_as_u12(state.program_counter.as_tuple()) + u12::new(1))),
        mc_result.symbol_table.get("TEST")
    );
}

#[test]
fn jump_taken_when_a_is_one() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("LIH [A == 1] TEST\nHLT\nLAB TEST\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(1);
    state.consume_until_halt();
    assert_eq!(
        Some(&tuple_as_u12(state.program_counter.as_tuple())),
        mc_result.symbol_table.get("TEST")
    );
}

#[test]
fn jump_not_taken_when_b_is_zero() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("LIH [B == 1] TEST\nHLT\nLAB TEST\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();
    assert_eq!(
        Some(&(tuple_as_u12(state.program_counter.as_tuple()) + u12::new(1))),
        mc_result.symbol_table.get("TEST")
    );
}

#[test]
fn jump_taken_when_b_is_one() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("LIH [B == 1] TEST\nHLT\nLAB TEST\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.b = u6::new(1);
    state.consume_until_halt();
    assert_eq!(
        Some(&tuple_as_u12(state.program_counter.as_tuple())),
        mc_result.symbol_table.get("TEST")
    );
}

#[test]
fn jump_not_taken_when_c_is_zero() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("LIH [C == 1] TEST\nHLT\nLAB TEST\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();
    assert_eq!(
        Some(&(tuple_as_u12(state.program_counter.as_tuple()) + u12::new(1))),
        mc_result.symbol_table.get("TEST")
    );
}

#[test]
fn jump_taken_when_c_is_one() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("LIH [C == 1] TEST\nHLT\nLAB TEST\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.c = u6::new(1);
    state.consume_until_halt();
    assert_eq!(
        Some(&tuple_as_u12(state.program_counter.as_tuple())),
        mc_result.symbol_table.get("TEST")
    );
}
