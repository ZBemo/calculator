#![warn(clippy::pedantic, clippy::all, clippy::perf)]

use calc_ir::{Number, Program};

#[cfg(test)]
mod test;

pub fn interpret_function(
    function: &str,
    program: &Program,
    arguments: Vec<Number>,
) -> Result<Number, ()> {
    let mut registers: Vec<Number> = Vec::new();
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
                registers[register.0] = *value;
            }
            Instruction::Call {
                name,
                arguments,
                out,
            } => {
                let result = interpret_function(
                    name,
                    program,
                    arguments.iter().map(|r| registers[r.0]).collect(),
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
            Instruction::JEqual { lhs, rhs, to } => {
                todo!("Implementing jumps!");
            }
            Instruction::JNonZero { check, to } => {
                todo!("Implementing jumps!");
            }
            Instruction::JZero { check, to } => {
                todo!("Implementing jumps!");
            }

            Instruction::Add { lhs, rhs, out } => {
                registers[out.0] = registers[lhs.0] + registers[rhs.0];
            }
            Instruction::Subtract { lhs, rhs, out } => {
                registers[out.0] = registers[lhs.0] - registers[rhs.0];
            }
            Instruction::Multiply { lhs, rhs, out } => {
                registers[out.0] = registers[lhs.0] * registers[rhs.0];
            }
            Instruction::Divide { lhs, rhs, out } => {
                registers[out.0] = registers[lhs.0] / registers[rhs.0];
            }
            Instruction::Modulo { lhs, rhs, out } => {
                registers[out.0] = registers[lhs.0] % registers[rhs.0];
            }

            Instruction::BitOr { lhs, rhs, out } => {
                registers[out.0] = registers[lhs.0] | registers[rhs.0];
            }
            Instruction::BitNotOr { lhs, rhs, out } => {
                registers[out.0] = registers[lhs.0] ^ registers[rhs.0];
            }
            Instruction::BitAnd { lhs, rhs, out } => {
                registers[out.0] = registers[lhs.0] & registers[rhs.0];
            }
            Instruction::ShiftL { lhs, rhs, out } => {
                registers[out.0] = registers[lhs.0] << registers[rhs.0];
            }
            Instruction::ShiftR { lhs, rhs, out } => {
                registers[out.0] = registers[lhs.0] >> registers[rhs.0];
            }

            Instruction::Invalid => panic!("Invalid instruction in function {}", function),

            // leave this here just in case
            #[allow(unreachable_patterns)]
            _ => todo!("unimplemented instruction"),
        };
    }

    todo!()
}
