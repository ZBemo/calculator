//! This crate contains a trait for a program built out of zach ir.
//! It also contains a basic implementation of [`Program`] in [`BasicProgram`],
//!
//! The [`calc_ir`] crate will accept any struct that implements [`Program`] to interpret, as well as the JIT for lowering,
//! however, the optimizer may require additional trait implementations in order to optimize code

use std::collections::HashMap;

type Instruction = crate::Instruction<BlockID, String>;

/// A trait for any program constructed out of Zach IR
/// Any structure which implements the two functions correctly will be able to be run through the interpreter and jit
///
/// See module documentation for more information about getting started
pub trait Program {
    /// The type used to identify a block, this can be anything as long as it can be used to look up a valid slice of [`Instruction`]s.
    ///
    /// It is necesarry that when block_pointer1 == block_pointer2, then get_ir(block_pointer1) == get_ir(block_pointer2), and thus consumers of a `Program`
    /// are allowed to assume that when two `BlockPointer`s compare equal, the blocks that they point to have the same content, although they may have
    /// different locations in memory.
    type BlockPointer: Eq;

    /// The type used to identify a function, this can be anything as long as it can be used to look up a BlockPointer to
    /// the correct function. It must also implement Eq for the consumer's benefit.
    ///
    /// It follows the same rules as [`BlockPointer`], where if function_pointer1 == function_pointer2,
    /// then get_function_entry(function_pointer1) == get_function_entry(function_pointer2).
    type FunctionPointer: Eq;

    /// Get a function's beginning block from its function pointer.
    /// If a given `function_id` is not registered to a function, then return None
    ///
    /// Important: see [`BlockPointer`] and [`FunctionPointer`] for rules that this function must follow about any given [`FunctionPointer`]
    //TODO: this shouldn't force reference to Self::FunctionPointer
    fn get_function_entry(&self, function_id: &Self::FunctionPointer)
        -> Option<Self::BlockPointer>;

    /// get a list of [`Instruction`]s, where \[0\] is the first instruction in the block pointed to by `block_id`.
    ///
    /// Important: see [`BlockPointer`] and [`FunctionPointer`] for rules that this function must follow about any given [`BlockPointer`]
    fn get_ir(
        &self,
        block_id: Self::BlockPointer,
    ) -> &[crate::Instruction<Self::BlockPointer, Self::FunctionPointer>];
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
