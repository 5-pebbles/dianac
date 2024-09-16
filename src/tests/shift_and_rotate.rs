use crate::{
    compilation::{compile_to_binary, CompileInfo},
    emulation::InteractiveState,
    test_builder,
};
use arbitrary_int::{u12, u6};

// ROL tests
test_builder!(
    rol_a,
    "ROL A\nHLT",
    |state| {
        state.a = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b011011));
    }
);

test_builder!(
    rol_b,
    "ROL B\nHLT",
    |state| {
        state.b = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b011011));
    }
);

test_builder!(
    rol_c,
    "ROL C\nHLT",
    |state| {
        state.c = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b011011));
    }
);

// ROR tests
test_builder!(
    ror_a,
    "ROR A\nHLT",
    |state| {
        state.a = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b110110));
    }
);

test_builder!(
    ror_b,
    "ROR B\nHLT",
    |state| {
        state.b = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b110110));
    }
);

test_builder!(
    ror_c,
    "ROR C\nHLT",
    |state| {
        state.c = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b110110));
    }
);

// SHL tests
test_builder!(
    shl_a,
    "SHL A\nHLT",
    |state| {
        state.a = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b011010));
    }
);

test_builder!(
    shl_b,
    "SHL B\nHLT",
    |state| {
        state.b = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b011010));
    }
);

test_builder!(
    shl_c,
    "SHL C\nHLT",
    |state| {
        state.c = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b011010));
    }
);

// SHR tests
test_builder!(
    shr_a,
    "SHR A\nHLT",
    |state| {
        state.a = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b010110));
    }
);

test_builder!(
    shr_b,
    "SHR B\nHLT",
    |state| {
        state.b = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b010110));
    }
);

test_builder!(
    shr_c,
    "SHR C\nHLT",
    |state| {
        state.c = u6::new(0b101101);
        state.consume_until_halt();
    },
    |state, _machine_code_result| {
        assert_eq!(state.c, u6::new(0b010110));
    }
);
