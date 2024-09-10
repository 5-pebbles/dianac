use arbitrary_int::{u12, u6};

use crate::{compilation::compile_to_binary, emulation::InteractiveState, utils::tuple_as_u12};

#[test]
fn jump_if_eq() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("LIH [A == 0] TEST\nHLT\nLAB TEST\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();
    assert_eq!(
        Some(&tuple_as_u12(state.program_counter.as_tuple())),
        mc_result.symbol_table.get("TEST")
    );
    // reset
    state = InteractiveState::new();
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(1);
    state.consume_until_halt();
    assert_eq!(
        Some(&(tuple_as_u12(state.program_counter.as_tuple()) + u12::new(1))),
        mc_result.symbol_table.get("TEST")
    );
}

#[test]
fn jump_if_not_eq() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("LIH [A != 0] TEST\nHLT\nLAB TEST\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();
    assert_eq!(
        Some(&(tuple_as_u12(state.program_counter.as_tuple()) + u12::new(1))),
        mc_result.symbol_table.get("TEST")
    );
    // reset
    state = InteractiveState::new();
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(1);
    state.consume_until_halt();
    assert_eq!(
        Some(&tuple_as_u12(state.program_counter.as_tuple())),
        mc_result.symbol_table.get("TEST")
    );
}

#[test]
fn multiple_jumps() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary(
        "LIH [A == 0] TEST1\nLIH [A != 0] TEST2\nHLT\nLAB TEST1\nHLT\nLAB TEST2\nHLT",
    );
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();
    assert_eq!(
        Some(&tuple_as_u12(state.program_counter.as_tuple())),
        mc_result.symbol_table.get("TEST1")
    );
    // reset
    state = InteractiveState::new();
    state.memory.store_array(0, &mc_result.binary);
    state.a = u6::new(1);
    state.consume_until_halt();
    assert_eq!(
        Some(&tuple_as_u12(state.program_counter.as_tuple())),
        mc_result.symbol_table.get("TEST2")
    );
}
