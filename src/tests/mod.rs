mod halt_and_nop;
mod lab_and_pc;
mod lih;
mod nor_and_or;
mod nxor_and_xor;
mod shift_and_rotate;

/// Quickly create compact tests!
#[macro_export]
macro_rules! test_builder {
    (
        $test_name:ident,
        $source:expr, |
        $mutation_state:ident |
        $mutation_closure:expr, |
        $assertion_state:ident,
        $assertion_machine_code_result:ident |
        $assertion_closure:expr
    ) => {
        #[test]
        fn $test_name() {
            let mut state = InteractiveState::new();
            let machine_code_result = compile_to_binary($source);
            assert_eq!(machine_code_result.diagnostics.len(), 0);
            state.memory.store_array(0, &machine_code_result.binary);
            let mutation_closure = |$mutation_state: &mut InteractiveState| $mutation_closure;
            mutation_closure(&mut state);
            let assertion_closure =
                |$assertion_state: InteractiveState,
                 $assertion_machine_code_result: CompileInfo| { $assertion_closure };
            assertion_closure(state, machine_code_result)
        }
    };
}
