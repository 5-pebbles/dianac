use crate::{
    compilation::{compile_to_binary, CompileInfo},
    emulation::InteractiveState,
    test_builder,
};
use arbitrary_int::{u12, u6};

test_builder!(
    nop,
    "NOP\nHLT",
    |state| {
        state.consume_until_halt();
    },
    |state, machine_code_result| {
        assert_eq!(machine_code_result.diagnostics.len(), 0);
        assert_eq!(
            (state.a, state.b, state.c),
            (u6::new(0), u6::new(0), u6::new(0))
        );
    }
);

test_builder!(
    halt,
    "HLT",
    |state| {
        state.consume_until_halt();
    },
    |state, machine_code_result| {
        assert_eq!(machine_code_result.diagnostics.len(), 0);
        assert_eq!(state.program_counter.as_tuple(), (u6::new(0), u6::new(0)));
        assert_eq!(
            (state.a, state.b, state.c),
            (u6::new(0), u6::new(0), u6::new(0))
        );
    }
);

test_builder!(
    halt_after_nop,
    "NOP\nHLT",
    |state| {
        state.consume_until_halt();
    },
    |state, machine_code_result| {
        assert_eq!(machine_code_result.diagnostics.len(), 0);
        assert_eq!(state.program_counter.as_tuple(), (u6::new(0), u6::new(1)));
    }
);

#[test]
fn hand_assembled_nor() {
    let mut state = InteractiveState::new();
    state.a = u6::new(0b101010); // garbage

    state.memory.store_array(0, &[
        u6::new(0b000011), // NOR A Immediate
        u6::new(0b111111), //   0b111111 (zero A)
        u6::new(0b000011), // NOR A Immediate
        u6::new(0b010101), //   0b010101 (load complement)
        u6::new(0b000000), // NOR A A   (NOT)
        u6::new(0b001111), // HLT
    ]);

    state.consume_until_halt();
    assert_eq!(state.a, u6::new(0b010101));
}
