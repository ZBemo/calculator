use calc_ir::{Instruction, Program, Register};

use self::structs::FlatProgram;
use std::hash::Hash;

pub mod graph;
pub mod passes;
pub mod structs;

pub use graph::Graph;

pub type Block<BPT, FPT> = Vec<Instruction<BPT, FPT>>;

//TODO: figure out how to apply optimizations, probably through a builder or smtn

fn depends_on_register<BlockPointerT: Eq + Clone, FunctionPointerT: Eq + Clone>(
    register: Register,
    instruction: &Instruction<BlockPointerT, FunctionPointerT>,
) -> bool {
    match instruction {
        // instructions that depend on one register, `r`
        Instruction::LoadImmediate(_, r)
        | Instruction::Ret(r)
        | Instruction::JNonZero { check: r, to: _ }
        | Instruction::JZero { check: r, to: _ } => *r == register,
        // instructions with a vector of registers
        Instruction::Call {
            function_id: _,
            arguments,
            out: _,
        } => arguments.iter().any(|r| *r == register),
        // instructions that don't depend on any register
        Instruction::LoadArgs(_) | Instruction::Jump(_) | Instruction::Invalid => false,
        // instructions that depend on a lhs and a rhs register
        Instruction::JEqual { lhs, rhs, to: _ }
        | Instruction::JNotEqual { lhs, rhs, to: _ }
        | Instruction::Add { lhs, rhs, out: _ }
        | Instruction::Subtract { lhs, rhs, out: _ }
        | Instruction::Multiply { lhs, rhs, out: _ }
        | Instruction::Divide { lhs, rhs, out: _ }
        | Instruction::Modulo { lhs, rhs, out: _ }
        | Instruction::BitOr { lhs, rhs, out: _ }
        | Instruction::BitNotOr { lhs, rhs, out: _ }
        | Instruction::BitAnd { lhs, rhs, out: _ }
        | Instruction::ShiftL { lhs, rhs, out: _ }
        | Instruction::ShiftR { lhs, rhs, out: _ } => *lhs == register || *rhs == register,
    }
}

/// Finds all Instructions that rely on the value of a given register, returning a vector of their indexes
fn find_dependencies<
    BlockPointerT: Eq + std::fmt::Debug + Clone,
    FunctionPointerT: Eq + std::fmt::Debug + Clone + Hash,
>(
    register: Register,
    block: &Block<BlockPointerT, FunctionPointerT>,
) -> Vec<usize> {
    block
        .iter()
        .enumerate()
        // only take ones that depend on `register`
        .filter(|(_, instruction)| depends_on_register(register, instruction))
        // map them to their indexes
        // maybe we should return Vec<&Instruction>
        // we'll see which is more convenient for real use
        .map(|(index, _)| index)
        .collect()
}

pub fn optimize_program<
    BlockPointerT: Eq + std::fmt::Debug + Clone,
    FunctionPointerT: Eq + std::fmt::Debug + Clone + Hash,
    ProgramT: Program<FunctionPointer = FunctionPointerT, BlockPointer = BlockPointerT>,
>(
    program: ProgramT,
) -> FlatProgram<FunctionPointerT> {
    todo!()
}
