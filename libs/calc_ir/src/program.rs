//! This crate contains a trait for a program built out of zach ir.
//! It also contains a basic implementation of [`Program`] in [`BasicProgram`],
//!
//! The [`calc_ir`] crate will accept any struct that implements [`Program`] to interpret, as well as the JIT for lowering,
//! however, the optimizer may require additional trait implementations in order to optimize code

use std::collections::HashMap;

type Instruction = crate::Instruction<BlockID>;

pub trait Program {
    type BlockPointer: Eq;
    type FunctionPointer;

    /// Get a function's beginning block from its function pointer, most likely a string.
    /// if a given `function_id` is not registered to a function, then returns None
    fn get_function_entry(&self, function_id: &Self::FunctionPointer)
        -> Option<Self::BlockPointer>;

    /// get a list of [`Instruction`]s, where \[0\] is the first instruction in the block pointed to by `block_id`
    fn get_ir(&self, block_id: Self::BlockPointer) -> &[crate::Instruction<Self::BlockPointer>];
}

/// A newtype wrapper around usize to force consumers to call [`get_function_entry()`] first, ensuring that we only ever lookup an
/// existing Block
///
/// this is only used in the implementation of [`BasicProgram`]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockID(pub(crate) usize);

/// A struct that implements [`Program`] in a simple way, the best way to acquire one of these is
/// through a [`builder::Program`]
#[allow(clippy::module_name_repetitions)]
pub struct BasicProgram {
    pub(crate) function_list: HashMap<String, BlockID>,
    // you could simplify this by having a Vec<Instruction> where a BlocKPointer is an offset to the first Instruction of the block
    // but this would make optimization far more complex.. it might be a better idea to have a pass at the end of the optimization
    // that "flattens" it from Vec<Vec<Instruction>> to Vec<Instruction> after the transformations have been made
    pub(crate) blocks: Vec<Vec<Instruction>>,
}

impl Program for BasicProgram {
    type FunctionPointer = String;
    type BlockPointer = BlockID;

    fn get_function_entry(
        &self,
        function_id: &Self::FunctionPointer,
    ) -> Option<Self::BlockPointer> {
        // Self::BlockPointer is trivially copiable
        self.function_list.get(function_id).map(Clone::clone)
    }

    /// Safety: `block_id` is only attainable through this module, and every path in this module must ensure that all
    /// publicly available [`BlockIDs`] are valid
    ///
    /// This should be guaranteed if you obtain your [`BasicProgram`] through a [`builder::Program`]
    fn get_ir(&self, block_id: Self::BlockPointer) -> &[Instruction] {
        // SAFETY: block_id is only attainable through this module, and every path in this module must ensure that all
        // publicly available BlockIDs are valid
        unsafe { self.blocks.get_unchecked(block_id.0) }
    }
}
