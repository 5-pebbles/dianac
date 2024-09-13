use crate::{
    compilation::{compile_to_binary, CompileInfo},
    test_builder, InteractiveState,
};
use arbitrary_int::{u12, u6};

macro_rules! test_bitwise_builder {
    (
        $test_name:ident,
        $keyword:expr, |
        $mutation_state:ident |
        $mutation_closure:expr,($($($reg:tt = $value:expr)?),*)
    ) => {
        test_builder!(
            $test_name,
            concat!($keyword, "\nHLT"),
            |$mutation_state| {$mutation_closure $mutation_state.consume_until_halt();},
            |state, _machine_code_result| {
                $($(assert_eq!(state.$reg, $value))?);*
            }
        );

    };
}

mod not {
    use super::*;

    test_bitwise_builder!(
        not_a,
        "NOT A",
        |state| {
            state.a = u6::new(0b101101);
        },
        (a = u6::new(0b010010), b = u6::new(0), c = u6::new(0))
    );

    test_bitwise_builder!(
        not_b,
        "NOT B",
        |state| {
            state.b = u6::new(0b101101);
        },
        (a = u6::new(0), b = u6::new(0b010010), c = u6::new(0))
    );

    test_bitwise_builder!(
        not_c,
        "NOT C",
        |state| {
            state.c = u6::new(0b101101);
        },
        (a = u6::new(0), b = u6::new(0), c = u6::new(0b010010))
    );

    test_bitwise_builder!(
        not_a_all_ones,
        "NOT A",
        |state| {
            state.a = u6::new(0b111111);
        },
        (a = u6::new(0), b = u6::new(0), c = u6::new(0))
    );

    test_bitwise_builder!(
        not_b_alternating,
        "NOT B",
        |state| {
            state.b = u6::new(0b101010);
        },
        (a = u6::new(0), b = u6::new(0b010101), c = u6::new(0))
    );

    test_bitwise_builder!(
        not_c_single_bit,
        "NOT C",
        |state| {
            state.c = u6::new(0b000001);
        },
        (a = u6::new(0), b = u6::new(0), c = u6::new(0b111110))
    );
}

mod and {
    use super::*;

    test_bitwise_builder!(
        and_ab,
        "AND A B",
        |state| {
            state.a = u6::new(0b101101);
            state.b = u6::new(0b110011);
        },
        (
            a = u6::new(0b100001),
            b = !u6::new(0b110011),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        and_bc,
        "AND B C",
        |state| {
            state.b = u6::new(0b101101);
            state.c = u6::new(0b110011);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b100001),
            c = !u6::new(0b110011)
        )
    );

    test_bitwise_builder!(
        and_ca,
        "AND C A",
        |state| {
            state.c = u6::new(0b101101);
            state.a = u6::new(0b110011);
        },
        (
            a = !u6::new(0b110011),
            b = u6::new(0b000000),
            c = u6::new(0b100001)
        )
    );

    test_bitwise_builder!(
        and_a_immediate,
        "AND A 0b110011",
        |state| {
            state.a = u6::new(0b101101);
        },
        (
            a = u6::new(0b100001),
            b = u6::new(0b000000),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        and_b_immediate,
        "AND B 0b110011",
        |state| {
            state.b = u6::new(0b101101);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b100001),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        and_c_immediate,
        "AND C 0b110011",
        |state| {
            state.c = u6::new(0b101101);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b000000),
            c = u6::new(0b100001)
        )
    );
}

mod nand {
    use super::*;

    test_bitwise_builder!(
        nand_ab,
        "NAND A B",
        |state| {
            state.a = u6::new(0b101101);
            state.b = u6::new(0b110011);
        },
        (
            a = u6::new(0b011110),
            b = !u6::new(0b110011),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        nand_bc,
        "NAND B C",
        |state| {
            state.b = u6::new(0b101101);
            state.c = u6::new(0b110011);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b011110),
            c = !u6::new(0b110011)
        )
    );

    test_bitwise_builder!(
        nand_ca,
        "NAND C A",
        |state| {
            state.c = u6::new(0b101101);
            state.a = u6::new(0b110011);
        },
        (
            a = !u6::new(0b110011),
            b = u6::new(0b000000),
            c = u6::new(0b011110)
        )
    );

    test_bitwise_builder!(
        nand_a_immediate,
        "NAND A 0b110011",
        |state| {
            state.a = u6::new(0b101101);
        },
        (
            a = u6::new(0b011110),
            b = u6::new(0b000000),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        nand_b_immediate,
        "NAND B 0b110011",
        |state| {
            state.b = u6::new(0b101101);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b011110),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        nand_c_immediate,
        "NAND C 0b110011",
        |state| {
            state.c = u6::new(0b101101);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b000000),
            c = u6::new(0b011110)
        )
    );
}

mod or {
    use super::*;

    test_bitwise_builder!(
        or_ab,
        "OR A B",
        |state| {
            state.a = u6::new(0b100101);
            state.b = u6::new(0b110011);
        },
        (
            a = u6::new(0b110111),
            b = u6::new(0b110011),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        or_bc,
        "OR B C",
        |state| {
            state.b = u6::new(0b100101);
            state.c = u6::new(0b110011);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b110111),
            c = u6::new(0b110011)
        )
    );

    test_bitwise_builder!(
        or_ca,
        "OR C A",
        |state| {
            state.c = u6::new(0b100101);
            state.a = u6::new(0b110011);
        },
        (
            a = u6::new(0b110011),
            b = u6::new(0b000000),
            c = u6::new(0b110111)
        )
    );

    test_bitwise_builder!(
        or_a_immediate,
        "OR A 0b110011",
        |state| {
            state.a = u6::new(0b100101);
        },
        (
            a = u6::new(0b110111),
            b = u6::new(0b000000),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        or_b_immediate,
        "OR B 0b110011",
        |state| {
            state.b = u6::new(0b100101);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b110111),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        or_c_immediate,
        "OR C 0b110011",
        |state| {
            state.c = u6::new(0b100101);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b000000),
            c = u6::new(0b110111)
        )
    );
}

mod nor {
    use super::*;

    test_bitwise_builder!(
        nor_ab,
        "NOR A B",
        |state| {
            state.a = u6::new(0b100101);
            state.b = u6::new(0b110011);
        },
        (
            a = u6::new(0b001000),
            b = u6::new(0b110011),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        nor_bc,
        "NOR B C",
        |state| {
            state.b = u6::new(0b100101);
            state.c = u6::new(0b110011);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b001000),
            c = u6::new(0b110011)
        )
    );

    test_bitwise_builder!(
        nor_ca,
        "NOR C A",
        |state| {
            state.c = u6::new(0b100101);
            state.a = u6::new(0b110011);
        },
        (
            a = u6::new(0b110011),
            b = u6::new(0b000000),
            c = u6::new(0b001000)
        )
    );

    test_bitwise_builder!(
        nor_a_immediate,
        "NOR A 0b110011",
        |state| {
            state.a = u6::new(0b100101);
        },
        (
            a = u6::new(0b001000),
            b = u6::new(0b000000),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        nor_b_immediate,
        "NOR B 0b110011",
        |state| {
            state.b = u6::new(0b100101);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b001000),
            c = u6::new(0b000000)
        )
    );

    test_bitwise_builder!(
        nor_c_immediate,
        "NOR C 0b110011",
        |state| {
            state.c = u6::new(0b100101);
        },
        (
            a = u6::new(0b000000),
            b = u6::new(0b000000),
            c = u6::new(0b001000)
        )
    );
}

mod xor {
    use super::*;

    test_bitwise_builder!(
        xor_ab,
        "XOR A B",
        |state| {
            state.a = u6::new(0b100101);
            state.b = u6::new(0b110011);
        },
        (a = u6::new(0b010110), b = u6::new(0b110011))
    );

    test_bitwise_builder!(
        xor_bc,
        "XOR B C",
        |state| {
            state.b = u6::new(0b100101);
            state.c = u6::new(0b110011);
        },
        (b = u6::new(0b010110), c = u6::new(0b110011))
    );

    test_bitwise_builder!(
        xor_ca,
        "XOR C A",
        |state| {
            state.c = u6::new(0b100101);
            state.a = u6::new(0b110011);
        },
        (a = u6::new(0b110011), c = u6::new(0b010110))
    );

    test_bitwise_builder!(
        xor_a_immediate,
        "XOR A 0b110011",
        |state| {
            state.a = u6::new(0b100101);
        },
        (a = u6::new(0b010110), b = u6::new(0b000000),)
    );

    test_bitwise_builder!(
        xor_b_immediate,
        "XOR B 0b110011",
        |state| {
            state.b = u6::new(0b100101);
        },
        (a = u6::new(0b000000), b = u6::new(0b010110),)
    );

    test_bitwise_builder!(
        xor_c_immediate,
        "XOR C 0b110011",
        |state| {
            state.c = u6::new(0b100101);
        },
        (a = u6::new(0b000000), c = u6::new(0b010110))
    );
}

mod nxor {
    use super::*;

    test_bitwise_builder!(
        nxor_ab,
        "NXOR A B",
        |state| {
            state.a = u6::new(0b100101);
            state.b = u6::new(0b110011);
        },
        (a = u6::new(0b101001), b = u6::new(0b110011))
    );

    test_bitwise_builder!(
        nxor_bc,
        "NXOR B C",
        |state| {
            state.b = u6::new(0b100101);
            state.c = u6::new(0b110011);
        },
        (b = u6::new(0b101001), c = u6::new(0b110011))
    );

    test_bitwise_builder!(
        nxor_ca,
        "NXOR C A",
        |state| {
            state.c = u6::new(0b100101);
            state.a = u6::new(0b110011);
        },
        (a = u6::new(0b110011), c = u6::new(0b101001))
    );

    test_bitwise_builder!(
        nxor_a_immediate,
        "NXOR A 0b110011",
        |state| {
            state.a = u6::new(0b100101);
        },
        (a = u6::new(0b101001), b = u6::new(0b000000),)
    );

    test_bitwise_builder!(
        nxor_b_immediate,
        "NXOR B 0b110011",
        |state| {
            state.b = u6::new(0b100101);
        },
        (a = u6::new(0b000000), b = u6::new(0b101001),)
    );

    test_bitwise_builder!(
        nxor_c_immediate,
        "NXOR C 0b110011",
        |state| {
            state.c = u6::new(0b100101);
        },
        (a = u6::new(0b000000), c = u6::new(0b101001))
    );
}
