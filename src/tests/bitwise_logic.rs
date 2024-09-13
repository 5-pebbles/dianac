use crate::{
    compilation::{compile_to_binary, CompileInfo},
    InteractiveState,
};
use arbitrary_int::{u12, u6};

use crate::test_builder;

// NOT

test_builder!(
    not_a,
    "NOT A",
    |state| {
        state.a = u6::new(0b101101);
        state.consume_instruction()
    },
    |state, _machine_code_result| {
        assert_eq!(state.a, u6::new(0b010010));
        assert_eq!(state.b, u6::new(0b000000));
        assert_eq!(state.c, u6::new(0b000000));
    }
);

test_builder!(
    not_b,
    "NOT B",
    |state| {
        state.b = u6::new(0b101101);
        state.consume_instruction()
    },
    |state, _machine_code_result| {
        assert_eq!(state.a, u6::new(0b000000));
        assert_eq!(state.b, u6::new(0b010010));
        assert_eq!(state.c, u6::new(0b000000));
    }
);
test_builder!(
    not_c,
    "NOT C",
    |state| {
        state.c = u6::new(0b101101);
        state.consume_instruction()
    },
    |state, _machine_code_result| {
        assert_eq!(state.a, u6::new(0b000000));
        assert_eq!(state.b, u6::new(0b000000));
        assert_eq!(state.c, u6::new(0b010010));
    }
);
