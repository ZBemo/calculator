#[allow(unused_imports)]
use calc_ir::{
    builder::instructions::{Arithmetic, BitWise, Jump},
    Builder,
};

// test that a basic function (adding 1 and 2) can be built and interpreted correctly
#[test]
fn basic_add() {
    const MAIN_FN_NAME: &str = "one_plus_two";
    let mut builder = Builder::new();

    let mut block_builder = builder.build_block();

    let one = block_builder.add_immediate(1);
    let two = block_builder.add_immediate(2);
    let to_ret = block_builder.add_arithmetic(Arithmetic::Add, one, two);
    block_builder.add_ret(to_ret);

    let block_id = block_builder.finalize();

    builder.register_function(block_id, MAIN_FN_NAME.to_string());

    let program = builder.finalize();

    let result = crate::interpret_function(MAIN_FN_NAME, &program, &[]);

    assert_eq!(result, Ok(3));
}

/// make the same  function as above and then call it from another function
#[test]
fn two_functions() {
    const CALED_FN_NAME: &str = "one_plus_two";
    let mut builder = Builder::new();

    let mut block_builder = builder.build_block();

    let one = block_builder.add_immediate(1);
    let two = block_builder.add_immediate(2);
    let to_ret = block_builder.add_arithmetic(Arithmetic::Add, one, two);
    block_builder.add_ret(to_ret);

    let block_id = block_builder.finalize();

    builder.register_function(block_id, CALED_FN_NAME.to_string());

    let mut snd_block_builder = builder.build_block();

    // load the one argument to the function into a register
    let args = snd_block_builder.add_loadargs(1);
    // load 1 into a register
    let one = snd_block_builder.add_immediate(1);

    let arg_minus_one = snd_block_builder.add_arithmetic(Arithmetic::Subtract, args[0], one);

    snd_block_builder.add_ret(arg_minus_one);

    //TODO: call that and  subtract one from its return, should evaluate to 2
}
