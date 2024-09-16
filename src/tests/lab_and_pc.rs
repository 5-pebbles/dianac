use crate::{
    compilation::{compile_to_binary, CompileInfo},
    emulation::InteractiveState,
    test_builder,
    utils::tuple_as_u12,
};
use arbitrary_int::{u12, u6};

test_builder!(
    pc,
    "PC 0 32",
    |state| {
        state.consume_instruction();
    },
    |state, _machine_code_result| {
        assert_eq!(state.program_counter.as_tuple(), (u6::new(0), u6::new(32)));
    }
);

test_builder!(
    lab,
    "LAB ZERO\nNOP\nNOP\nLAB TWO",
    |_state| {},
    |_state, machine_code_result| {
        assert_eq!(
            machine_code_result.symbol_table.get("ZERO"),
            Some(&u12::new(0))
        );
        assert_eq!(
            machine_code_result.symbol_table.get("TWO"),
            Some(&u12::new(2))
        );
    }
);

test_builder!(
    pc_to_lab,
    "PC TEST\nNOP\nLAB TEST",
    |state| {
        state.consume_instruction();
    },
    |state, machine_code_result| {
        assert_eq!(
            Some(&tuple_as_u12(state.program_counter.as_tuple())),
            machine_code_result.symbol_table.get("TEST")
        );
    }
);
