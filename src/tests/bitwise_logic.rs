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

// AND

test_builder!(
    and_ab,
    "AND A B\nHLT",
    |state| {
        state.a = u6::new(0b101101);
        state.b = u6::new(0b110011);
        state.consume_until_halt()
    },
    |state, _machine_code_result| {
        assert_eq!(state.a, u6::new(0b100001));
        assert_eq!(state.b, !u6::new(0b110011));
        assert_eq!(state.c, u6::new(0b000000));
    }
);

test_builder!(
    and_bc,
    "AND B C\nHLT",
    |state| {
        state.b = u6::new(0b101101);
        state.c = u6::new(0b110011);
        state.consume_until_halt()
    },
    |state, _machine_code_result| {
        assert_eq!(state.a, u6::new(0b000000));
        assert_eq!(state.b, u6::new(0b100001));
        assert_eq!(state.c, !u6::new(0b110011));
    }
);

test_builder!(
    and_ca,
    "AND C A\nHLT",
    |state| {
        state.c = u6::new(0b101101);
        state.a = u6::new(0b110011);
        state.consume_until_halt()
    },
    |state, _machine_code_result| {
        assert_eq!(state.a, !u6::new(0b110011));
        assert_eq!(state.b, u6::new(0b000000));
        assert_eq!(state.c, u6::new(0b100001));
    }
);

test_builder!(
    and_a_immediate,
    "AND A 0b110011\nHLT",
    |state| {
        state.a = u6::new(0b101101);
        state.consume_until_halt()
    },
    |state, _machine_code_result| {
        assert_eq!(state.a, u6::new(0b100001));
        assert_eq!(state.b, u6::new(0b000000));
        assert_eq!(state.c, u6::new(0b000000));
    }
);

test_builder!(
    and_b_immediate,
    "AND B 0b110011\nHLT",
    |state| {
        state.b = u6::new(0b101101);
        state.consume_until_halt()
    },
    |state, _machine_code_result| {
        assert_eq!(state.a, u6::new(0b000000));
        assert_eq!(state.b, u6::new(0b100001));
        assert_eq!(state.c, u6::new(0b000000));
    }
);

test_builder!(
    and_c_immediate,
    "AND C 0b110011\nHLT",
    |state| {
        state.c = u6::new(0b101101);
        state.consume_until_halt()
    },
    |state, _machine_code_result| {
        assert_eq!(state.a, u6::new(0b000000));
        assert_eq!(state.b, u6::new(0b000000));
        assert_eq!(state.c, u6::new(0b100001));
    }
);
