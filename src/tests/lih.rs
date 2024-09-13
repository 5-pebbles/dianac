use crate::{
    compilation::{compile_to_binary, CompileInfo},
    utils::tuple_as_u12,
    InteractiveState,
};
use arbitrary_int::{u12, u6};

use crate::test_builder;

macro_rules! test_matrix_helper {
    ($test_name:ident, $condition:expr,($a:expr, $b:expr, $c:expr), $result:tt) => {
        test_builder!(
            $test_name,
            concat!("LIH ", $condition, " TEST\nHLT\nLAB TEST\nHLT"),
            |state| {
                state.a = u6::new($a);
                state.b = u6::new($b);
                state.c = u6::new($c);
                state.consume_until_halt()
            },
            |state, machine_code_result| {
                assert_eq!(
                    Some(
                        tuple_as_u12(state.program_counter.as_tuple()) + u12::new(1)
                            - u12::new($result)
                    ),
                    machine_code_result.symbol_table.get("TEST").cloned()
                );
            }
        );
    };
}

#[rustfmt::skip]
macro_rules! test_matrix_builder {
    (
        $condition:expr,[$less:tt, $equal:tt, $greater:tt]
    ) => {
        // A and 1
        test_matrix_helper!(when_a_is_less_then_one, concat!("[A ", $condition, " 1]"), (0, 0, 0), $less);
        test_matrix_helper!(when_one_is_less_then_a, concat!("[1 ", $condition, " A]"), (0, 0, 0), $greater);
        test_matrix_helper!(when_a_is_equal_then_one, concat!("[A ", $condition, " 1]"), (1, 0, 0), $equal);
        test_matrix_helper!(when_one_is_equal_then_a, concat!("[1 ", $condition, " A]"), (1, 0, 0), $equal);
        test_matrix_helper!(when_a_is_greater_then_one, concat!("[A ", $condition, " 1]"), (2, 0, 0), $greater);
        test_matrix_helper!(when_one_is_greater_then_a, concat!("[1 ", $condition, " A]"), (2, 0, 0), $less);

        // B and 1
        test_matrix_helper!(when_b_is_less_then_one, concat!("[B ", $condition, " 1]"), (0, 0, 0), $less);
        test_matrix_helper!(when_one_is_less_then_b, concat!("[1 ", $condition, " B]"), (0, 0, 0), $greater);
        test_matrix_helper!(when_b_is_equal_then_one, concat!("[B ", $condition, " 1]"), (0, 1, 0), $equal);
        test_matrix_helper!(when_one_is_equal_then_b, concat!("[1 ", $condition, " B]"), (0, 1, 0), $equal);
        test_matrix_helper!(when_b_is_greater_then_one, concat!("[B ", $condition, " 1]"), (0, 2, 0), $greater);
        test_matrix_helper!(when_one_is_greater_then_b, concat!("[1 ", $condition, " B]"), (0, 2, 0), $less);

        // C and 1
        test_matrix_helper!(when_c_is_less_then_one, concat!("[C ", $condition, " 1]"), (0, 0, 0), $less);
        test_matrix_helper!(when_one_is_less_then_c, concat!("[1 ", $condition, " C]"), (0, 0, 0), $greater);
        test_matrix_helper!(when_c_is_equal_then_one, concat!("[C ", $condition, " 1]"), (0, 0, 1), $equal);
        test_matrix_helper!(when_one_is_equal_then_c, concat!("[1 ", $condition, " C]"), (0, 0, 1), $equal);
        test_matrix_helper!(when_c_is_greater_then_one, concat!("[C ", $condition, " 1]"), (0, 0, 2), $greater);
        test_matrix_helper!(when_one_is_greater_then_c, concat!("[1 ", $condition, " C]"), (0, 0, 2), $less);

        // A and B
        test_matrix_helper!(when_a_is_less_than_b, concat!("[A ", $condition, " B]"), (1, 2, 0), $less);
        test_matrix_helper!(when_b_is_less_than_a, concat!("[B ", $condition, " A]"), (2, 1, 0), $less);
        test_matrix_helper!(when_a_equals_b, concat!("[A ", $condition, " B]"), (2, 2, 0), $equal);
        test_matrix_helper!(when_b_equals_a, concat!("[B ", $condition, " A]"), (2, 2, 0), $equal);
        test_matrix_helper!(when_a_is_greater_than_b, concat!("[A ", $condition, " B]"), (3, 2, 0), $greater);
        test_matrix_helper!(when_b_is_greater_than_a, concat!("[B ", $condition, " A]"), (2, 3, 0), $greater);

        // A and C
        test_matrix_helper!(when_a_is_less_than_c, concat!("[A ", $condition, " C]"), (1, 0, 2), $less);
        test_matrix_helper!(when_c_is_less_than_a, concat!("[C ", $condition, " A]"), (2, 0, 1), $less);
        test_matrix_helper!(when_a_equals_c, concat!("[A ", $condition, " C]"), (2, 0, 2), $equal);
        test_matrix_helper!(when_c_equals_a, concat!("[C ", $condition, " A]"), (2, 0, 2), $equal);
        test_matrix_helper!(when_a_is_greater_than_c, concat!("[A ", $condition, " C]"), (3, 0, 2), $greater);
        test_matrix_helper!(when_c_is_greater_than_a, concat!("[C ", $condition, " A]"), (2, 0, 3), $greater);

        // B and C
        test_matrix_helper!(when_b_is_less_than_c, concat!("[B ", $condition, " C]"), (0, 1, 2), $less);
        test_matrix_helper!(when_c_is_less_than_b, concat!("[C ", $condition, " B]"), (0, 2, 1), $less);
        test_matrix_helper!(when_b_equals_c, concat!("[B ", $condition, " C]"), (0, 2, 2), $equal);
        test_matrix_helper!(when_c_equals_b, concat!("[C ", $condition, " B]"), (0, 2, 2), $equal);
        test_matrix_helper!(when_b_is_greater_than_c, concat!("[B ", $condition, " C]"), (0, 3, 2), $greater);
        test_matrix_helper!(when_c_is_greater_than_b, concat!("[C ", $condition, " B]"), (0, 2, 3), $greater);

        // 1 and 2
        test_matrix_helper!(when_one_is_less_than_two, concat!("[1 ", $condition, " 2]"), (0, 0, 0), $less);
        test_matrix_helper!(when_two_is_greater_than_one, concat!("[2 ", $condition, " 1]"), (0, 0, 0), $greater);

        // 2 and 2
        test_matrix_helper!(when_two_equals_two, concat!("[2 ", $condition, " 2]"), (0, 0, 0), $equal);

        // 3 and 2
        test_matrix_helper!(when_three_is_greater_than_two, concat!("[3 ", $condition, " 2]"), (0, 0, 0), $greater);
        test_matrix_helper!(when_two_is_less_than_three, concat!("[2 ", $condition, " 3]"), (0, 0, 0), $less);

        // 0 and 1
        test_matrix_helper!(when_zero_is_less_than_one, concat!("[0 ", $condition, " 1]"), (0, 0, 0), $less);
        test_matrix_helper!(when_one_is_greater_than_zero, concat!("[1 ", $condition, " 0]"), (0, 0, 0), $greater);

        // 63 and 0 (testing edge cases)
        test_matrix_helper!(when_sixtythree_is_greater_than_zero, concat!("[63 ", $condition, " 0]"), (0, 0, 0), $greater);
        test_matrix_helper!(when_zero_is_less_than_sixtythree, concat!("[0 ", $condition, " 63]"), (0, 0, 0), $less);

        // 0 and 0 (testing minimum value)
        test_matrix_helper!(when_zero_equals_zero, concat!("[0 ", $condition, " 0]"), (0, 0, 0), $equal);

        // 63 and 63 (testing maximum value)
        test_matrix_helper!(when_sixtythree_equals_sixtythree, concat!("[63 ", $condition, " 63]"), (0, 0, 0), $equal);
    };
}

mod equal {
    use super::*;
    test_matrix_builder!("==", [0, 1, 0]);
}

mod not_equal {
    use super::*;
    test_matrix_builder!("!=", [1, 0, 1]);
}

mod greater {
    use super::*;
    test_matrix_builder!(">", [0, 0, 1]);
}

mod greater_or_equal {
    use super::*;
    test_matrix_builder!(">=", [0, 1, 1]);
}

mod less {
    use super::*;
    test_matrix_builder!("<", [1, 0, 0]);
}

mod less_or_equal {
    use super::*;
    test_matrix_builder!("<=", [1, 1, 0]);
}
