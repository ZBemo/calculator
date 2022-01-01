//! This crate contains a trait for a program built out of zach ir.
//! It also contains a basic implementation of [`Program`] in [`implementations::BasicProgram`],
//!
//! The `calc_interpeter` crate will accept any struct that implements [`Program`] to interpret, as well as the JIT for lowering,
//! however, the optimizer may require additional trait implementations in order to optimize code

/// A trait for any program constructed out of Zach IR
/// Any structure which implements the two functions correctly will be able to be run through the interpreter and jit
///
/// See module documentation for more information about getting started
pub trait Program {
    /// The type used to identify a block, this can be anything as long as it can be used to look up a valid slice of [`crate::Instruction`]s.
    ///
    /// It is necesarry that when two block pointers, for example, `b1` and `b2` compare equal, then `Self::get_ir(b1)` == `Self::get_ir(b2)`,
    /// and thus consumers of a `Program` are allowed to assume that when two `BlockPointer`s compare equal, the blocks that they point to
    /// have the same content, although they may have different locations in memory.
    type BlockPointer: Eq + Clone;

    /// The type used to identify a function, this can be anything as long as it can be used to look up a BlockPointer to
    /// the correct function. It must also implement Eq for the consumer's benefit.
    ///
    /// This type must follow the a similar rule to [`Self::BlockPointer`], where if two [`Self::FunctionPointer`]s, `f1` and `f2` compare equal, then
    /// `Self::get_function_entry(f1)` must compare equal to `Self::get_function_entry(f2)`, and thus must also follow the rules establised in the documentation of
    /// [`Self::BlockPointer`]
    type FunctionPointer: Eq + Clone;

    /// Get a function's beginning block from its function pointer.
    /// If a given `function_id` is not registered to a function, then return None
    ///
    /// Important: see [`Self::FunctionPointer`] for rules that this function must follow about any given `FunctionPointer`
    fn get_function_entry(&self, function_id: &Self::FunctionPointer)
        -> Option<Self::BlockPointer>;

    /// get a Vec of [`crate::Instruction`]s, where \[0\] is the first instruction in the block pointed to by `block_id`.
    ///
    /// Important: see [`Self::BlockPointer`] for rules that this function must follow about any given `BlockPointer`
    fn get_ir(
        &self,
        block_id: &Self::BlockPointer,
    ) -> &[crate::Instruction<Self::BlockPointer, Self::FunctionPointer>];
}

/// A module with structures necesarry for the implementation of [`crate::builder::Program`]
pub mod implementations {
    use crate::Program;
    use std::collections::HashMap;
    /// A non-generic, solid implementation of [`crate::Instruction`], used to build this program
    type Instruction = crate::Instruction<BlockID, String>;

    /// A newtype wrapper around usize to force consumers to call [`Program::get_function_entry()`] first, ensuring that we only ever lookup an
    /// existing Block
    ///
    /// this is only used in the implementation of [`BasicProgram`]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct BlockID(pub(crate) usize);

    /// A struct that implements [`Program`] in a simple way, the best way to acquire one of these is
    /// through a [`crate::builder::Program`]
    #[allow(clippy::module_name_repetitions)]
    pub struct BasicProgram {
        pub(crate) function_list: HashMap<String, BlockID>,
        // you could simplify this by having a Vec<Instruction> where a BlocKPointer is an offset to the first Instruction of the block
        // but this would make optimization far more complex.. it might be a good idea to have a pass at the end of the optimization
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
        /// publicly available [`BlockID`]s are valid
        ///
        /// This should be guaranteed if you obtain your [`BasicProgram`] through a [`crate::builder::Program`]
        fn get_ir(&self, block_id: &Self::BlockPointer) -> &[Instruction] {
            // SAFETY: block_id is only attainable through this module, and every path in this module must ensure that all
            // publicly available BlockIDs are valid
            unsafe { self.blocks.get_unchecked(block_id.0) }
        }
    }
}
