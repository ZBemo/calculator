#[allow(unused_imports)]
use calc_ir::{builder::instructions::*, builder::Program};

// test that a basic function (adding 1 and 2) can be built and interpreted correctly
#[test]
fn basic_add() {
    const MAIN_FN_NAME: &str = "one_plus_two";
    let mut builder = Program::new();

    let mut main_function = builder.make_fn(MAIN_FN_NAME.to_string());

    let mut main_block = main_function.build_block();
    let one = main_block.add_immediate(1);
    let two = main_block.add_immediate(2);

    let one_plus_two = main_block.add_arithmetic(Arithmetic::Add, one, two);
    // should be three
    main_block.add_ret(one_plus_two);

    //TOOD: there's gotta be a better way to do this man
    let (id, main_function) = main_block.finalize();

    main_function.finalize(id);

    let program = builder.finalize();
    let result = crate::interpret_function(&MAIN_FN_NAME.to_string(), &program, &[]);

    assert_eq!(result, Ok(3))
}

/// make the same  function as above and then call it from another function
#[test]
fn two_functions() {
    const FIRST_FUNCTION_NAME: &str = "one_plus_two";
    const MAIN_FUNCTION_NAME: &str = "one_plus_two_minus_two";

    let mut builder = Program::new();

    let mut called_function = builder.make_fn(FIRST_FUNCTION_NAME.to_string());

    let mut entry_block = called_function.build_block();
    let one = entry_block.add_immediate(1);
    let two = entry_block.add_immediate(2);

    let one_plus_two = entry_block.add_arithmetic(Arithmetic::Add, one, two);
    // should be three
    entry_block.add_ret(one_plus_two);

    //TOOD: there's gotta be a better way to do this man
    let (id, called_function) = entry_block.finalize();

    let builder = called_function.finalize(id);

    let mut main_function = builder.make_fn(MAIN_FUNCTION_NAME.to_string());

    let mut entry_block = main_function.build_block();
    let two = entry_block.add_immediate(2);
    let call = entry_block.add_fn_call(FIRST_FUNCTION_NAME.clone().to_string(), Vec::new());
    let ret_minus_two = entry_block.add_arithmetic(Arithmetic::Subtract, call, two);
    entry_block.add_ret(ret_minus_two);

    let (entry_block_id, main_function) = entry_block.finalize();
    let builder = main_function.finalize(entry_block_id);

    let program = builder.finalize();

    let result = crate::interpret_function(&MAIN_FUNCTION_NAME.to_string(), &program, &[]);

    assert_eq!(result, Ok(1))
}
