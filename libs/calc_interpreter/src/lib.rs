#![warn(clippy::pedantic, clippy::all, clippy::perf)]

//! A basic interpreter for programs built by [`calc_ir`]

use calc_ir::{Number, Program};

type State = Vec<Number>;

#[cfg(test)]
mod test;

/// interprets a block, returning Some(Number) if [`calc_ir::Instruction::Ret`] is called,
/// otherwise returns None if end of block is reached with no return
fn interpret_block<
    BlockPointerT: Eq + std::fmt::Debug + Clone,
    FunctionPointerT: Eq + std::fmt::Debug + Clone,
    ProgramT: Program<FunctionPointer = FunctionPointerT, BlockPointer = BlockPointerT>,
>(
    block: &ProgramT::BlockPointer,
    program: &ProgramT,
    state: &mut State,
    arguments: Option<&[Number]>,
) -> Option<Number> {
    // yeah this is bad code idcidc
    let registers = state;
    let to_interpret = program.get_ir(block);

    for instruction in to_interpret {
        use calc_ir::Instruction;
        match instruction {
            Instruction::LoadImmediate(value, register) => {
                registers.insert(register.0, *value);
            }
            Instruction::Call {
                function_id,
                arguments,
                out,
            } => {
                // look up all arguments before passing them
                let arguments: Vec<Number> = arguments.iter().map(|r| registers[r.0]).collect();
                let result = interpret_function(function_id, program, &*arguments);
                registers.insert(out.0, result.unwrap());
            }
            Instruction::Ret(register) => return Some(registers[register.0]),
            Instruction::LoadArgs(load_into) => match arguments {
                Some(arguments) => {
                    let _ = load_into
                        .iter()
                        .zip(0..)
                        .map(|(r, i)| registers.insert(r.0, arguments[i]));
                }
                None => panic!("attempting to load arguments outside of a function call!"),
            },

            // TODO: figure out how to handle jumps
            // we should probably refactor 'interpret_function' into interpret_function and interpret_block
            Instruction::Jump(to) => {
                interpret_block(to, program, registers, arguments);
            }
            Instruction::JEqual { lhs, rhs, to } => {
                if registers[lhs.0] == registers[rhs.0] {
                    match interpret_block(to, program, registers, arguments) {
                        Some(ret) => return Some(ret),
                        None => continue,
                    }
                };
            }
            Instruction::JNotEqual { lhs, rhs, to } => {
                if registers[lhs.0] != registers[rhs.0] {
                    match interpret_block(to, program, registers, arguments) {
                        Some(ret) => return Some(ret),
                        None => continue,
                    }
                };
            }
            Instruction::JNonZero { check, to } => {
                if registers[check.0] != 0 {
                    match interpret_block(to, program, registers, arguments) {
                        Some(ret) => return Some(ret),
                        None => continue,
                    }
                };
            }
            Instruction::JZero { check, to } => {
                if registers[check.0] == 0 {
                    match interpret_block(to, program, registers, arguments) {
                        Some(ret) => return Some(ret),
                        None => continue,
                    }
                };
            }

            Instruction::Add { lhs, rhs, out } => {
                registers.insert(out.0, registers[lhs.0] + registers[rhs.0]);
            }
            Instruction::Subtract { lhs, rhs, out } => {
                registers.insert(out.0, registers[lhs.0] - registers[rhs.0]);
            }
            Instruction::Multiply { lhs, rhs, out } => {
                registers.insert(out.0, registers[lhs.0] * registers[rhs.0]);
            }
            Instruction::Divide { lhs, rhs, out } => {
                registers.insert(out.0, registers[lhs.0] / registers[rhs.0]);
            }
            Instruction::Modulo { lhs, rhs, out } => {
                registers.insert(out.0, registers[lhs.0] % registers[rhs.0]);
            }

            Instruction::BitOr { lhs, rhs, out } => {
                registers.insert(out.0, registers[lhs.0] | registers[rhs.0]);
            }
            Instruction::BitNotOr { lhs, rhs, out } => {
                registers.insert(out.0, registers[lhs.0] ^ registers[rhs.0]);
            }
            Instruction::BitAnd { lhs, rhs, out } => {
                registers.insert(out.0, registers[lhs.0] & registers[rhs.0]);
            }
            Instruction::ShiftL { lhs, rhs, out } => {
                registers.insert(out.0, registers[lhs.0] << registers[rhs.0]);
            }
            Instruction::ShiftR { lhs, rhs, out } => {
                registers.insert(out.0, registers[lhs.0] >> registers[rhs.0]);
            }

            Instruction::Invalid => {
                panic!("Invalid instruction in block with pointer {:?}", &block)
            }
        };
    }

    // end of block with no ret or jump
    // TODO: this implicitly continues the calling block, but this might not be desired behaviour
    None
}

/// interprets a function that's been registered to `program` with the name `function`, passing in the arguments in `arguments` and returns its result,
/// as returned by [`calc_ir::Instruction::Ret`]
///
/// # Errors
/// The function can fail in various ways, such as if it's told to interpret a function that doesn't exist
// TODO: Proper error type
pub fn interpret_function<
    BlockPointerT: Eq + std::fmt::Debug + Clone,
    FunctionPointerT: Eq + std::fmt::Debug + Clone,
    ProgramT: Program<FunctionPointer = FunctionPointerT, BlockPointer = BlockPointerT>,
>(
    function: &ProgramT::FunctionPointer,
    program: &ProgramT,
    arguments: &[Number],
) -> Result<Number, ()> {
    let mut registers: State = Vec::new();
    let to_interpret = {
        match program.get_function_entry(function) {
            Some(function_block) => function_block,
            None => return Err(()),
        }
    };

    match interpret_block(&to_interpret, program, &mut registers, Some(arguments)) {
        Some(num) => Ok(num),
        None => Err(()),
    }
}
