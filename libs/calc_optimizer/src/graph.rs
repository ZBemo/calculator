/// The graph built from a program
use crate::Block;
use std::collections::HashMap;
use std::hash::Hash;

use calc_ir::Program;

pub struct Graph<FunctionPointerT: Eq + std::fmt::Debug + Clone + Hash> {
    functions: HashMap<FunctionPointerT, Block<FunctionPointerT, usize>>,
}

impl<FunctionPointerT: Eq + Clone + Hash + std::fmt::Debug> Graph<FunctionPointerT> {
    fn from_program<
        BlockPointerT: Eq + std::fmt::Debug + Into<usize>,
        ProgramT: Program<BlockPointer = BlockPointerT, FunctionPointer = FunctionPointerT>,
    >(
        from: ProgramT,
        entry_pointers: Vec<FunctionPointerT>,
    ) -> Self {
        todo!()
    }
}
