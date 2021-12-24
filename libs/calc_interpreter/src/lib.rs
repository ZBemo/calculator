#![warn(clippy::pedantic, clippy::all, clippy::perf)]

//! A basic interpreter for programs built by [`calc_ir`]

use std::ops::Deref;

use calc_ir::{BlockId, Number, Program};

type State = Vec<Number>;

#[cfg(test)]
mod test;

/// interprets a block, returning Some(Number) if [`calc_ir::Instruction::Ret`] is called,
/// otherwise returns None if end of block is reached with no return
fn interpret_block(block: BlockId, program: &Program, state: State) -> Option<Number> {
    todo!()
}

/// interprets a function that's been registered to `program` with the name `function`, passing in the arguments in `arguments` and returns its result,
/// as returned by [`calc_ir::Instruction::Ret`]
///
/// # Errors
/// The function can fail in various ways, such as if it's told to interpret a function that doesn't exist
pub fn interpret_function(
    function: &str,
    program: &Program,
    arguments: &[Number],
) -> Result<Number, ()> {
    let mut registers: State = Vec::new();
    let to_interpret = {
        match program.lookup_function(&*function) {
            Some(function_block) => program.get_block(function_block),
            None => return Err(()),
        }
    };

    for instruction in to_interpret {
        use calc_ir::Instruction;
        match instruction {
            Instruction::LoadImmediate(value, register) => {
                registers.insert(register.0, *value);
            }
            Instruction::Call {
                name,
                arguments,
                out,
            } => {
                // rust complains if we try to use * on the call change, probably because of wacky precidence
                #[allow(clippy::explicit_deref_methods)]
                let result = interpret_function(
                    name,
                    program,
                    arguments
                        .iter()
                        .map(|r| registers[r.0])
                        .collect::<Vec<_>>()
                        .deref(),
                )?;
                registers[out.0] = result;
            }
            Instruction::Ret(register) => return Ok(registers[register.0]),
            Instruction::LoadArgs(load_into) => {
                // load arguments[i] into corresponding register r
                let _ = load_into
                    .iter()
                    .zip(0..)
                    .map(|(r, i)| registers[r.0] = arguments[i]);
            }

            // TODO: figure out how to handle jumps
            // we should probably refactor 'interpret_function' into interpret_function and interpret_block
            Instruction::Jump(to) => {
                todo!("Implementing jumps!");
            }
            Instruction::JEqual { lhs, rhs, to } => {
                todo!("Implementing jumps!");
            }
            Instruction::JNotEqual { lhs, rhs, to } => {
                todo!("Implementing jumps!");
            }
            Instruction::JNonZero { check, to } => {
                todo!("Implementing jumps!");
            }
            Instruction::JZero { check, to } => {
                todo!("Implementing jumps!");
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

            Instruction::Invalid => panic!("Invalid instruction in function {}", function),
        };
    }

    todo!()
}
