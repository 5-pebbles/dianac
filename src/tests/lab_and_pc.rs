use arbitrary_int::{u12, u6};

use crate::{compilation::compile_to_binary, emulation::InteractiveState, utils::tuple_as_u12};

#[test]
fn pc() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("PC 0 32");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_instruction();
    assert_eq!(state.program_counter.as_tuple(), (u6::new(0), u6::new(32)));
}

#[test]
fn lab() {
    let mc_result = compile_to_binary("LAB ZERO\nNOP\nNOP\nLAB TWO");
    assert_eq!(mc_result.diagnostics.len(), 0);
    assert_eq!(mc_result.symbol_table.get("ZERO"), Some(&u12::new(0)));
    assert_eq!(mc_result.symbol_table.get("TWO"), Some(&u12::new(2)));
}

#[test]
fn pc_to_lab() {
    let mut state = InteractiveState::new();
    let mc_result = compile_to_binary("PC TEST\nNOP\nLAB TEST");
    assert_eq!(mc_result.diagnostics.len(), 0);
    state.memory.store_array(0, &mc_result.binary);
    state.consume_instruction();
    assert_eq!(
        Some(&tuple_as_u12(state.program_counter.as_tuple())),
        mc_result.symbol_table.get("TEST")
    );
}
