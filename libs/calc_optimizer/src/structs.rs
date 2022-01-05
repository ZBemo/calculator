use std::collections::HashMap;
use std::hash::Hash;

use calc_ir::{Instruction, Program};

pub struct FlatProgram<FunctionPointerT: Eq + std::fmt::Debug + Clone + Hash> {
    function_pointer_map: HashMap<FunctionPointerT, usize>,
    all_instructions: Vec<Instruction<usize, FunctionPointerT>>,
}

impl<FunctionPointerT: Eq + std::fmt::Debug + Clone + Hash> Program
    for FlatProgram<FunctionPointerT>
{
    type BlockPointer = usize;

    type FunctionPointer = FunctionPointerT;

    fn get_function_entry(
        &self,
        function_id: &Self::FunctionPointer,
    ) -> Option<Self::BlockPointer> {
        self.function_pointer_map.get(function_id).map(Clone::clone)
    }

    fn get_ir(
        &self,
        block_id: &Self::BlockPointer,
    ) -> &[calc_ir::Instruction<Self::BlockPointer, Self::FunctionPointer>] {
        &self.all_instructions[*block_id..self.all_instructions.len()]
    }
}

/*

<
        BlockPointerT: Eq + Clone,
        FunctionPointerT: Eq + Clone + Hash + std::fmt::Debug,
        ProgramT: Program<BlockPointer = BlockPointerT, FunctionPointer = FunctionPointerT>,
    >
    */
