use arbitrary_int::u6;

use crate::{compilation::compile_to_binary, emulation::InteractiveState};

#[test]
fn nop() {
    let mut state = InteractiveState::new();

    let mc_result = compile_to_binary("NOP");
    assert_eq!(mc_result.diagnostics.len(), 0);

    state.memory.store_array(0, &mc_result.binary);
    state.consume_instruction();

    assert_eq!(
        (state.a, state.b, state.c),
        (u6::new(0), u6::new(0), u6::new(0))
    );
}

#[test]
fn halt() {
    let mut state = InteractiveState::new();

    let mc_result = compile_to_binary("HLT");
    assert_eq!(mc_result.diagnostics.len(), 0);

    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();

    assert_eq!(state.program_counter.as_tuple(), (u6::new(0), u6::new(0)));
    assert_eq!(
        (state.a, state.b, state.c),
        (u6::new(0), u6::new(0), u6::new(0))
    );
}

#[test]
fn halt_after_nop() {
    let mut state = InteractiveState::new();

    let mc_result = compile_to_binary("NOP\nHLT");
    assert_eq!(mc_result.diagnostics.len(), 0);

    state.memory.store_array(0, &mc_result.binary);
    state.consume_until_halt();

    assert_eq!(state.program_counter.as_tuple(), (u6::new(0), u6::new(1)));
}
