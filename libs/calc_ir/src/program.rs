// TODO: fix Program's permissions, they're wack right now

use crate::{BlockId, Instruction};
use std::collections::HashMap;

/// A "block" of code, useful for loops, if commands, etc
pub type Block = Vec<Instruction>;

///TODO:  this probably doesn't belong in builder, move to lib.rs
/// a collection of Blocks and functions, built by a [`super::Builder`]
pub struct Program {
    pub(super) blocks: Vec<Block>,
    pub(super) functions: HashMap<String, BlockId>,
}

impl Program {
    #[must_use]
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            functions: HashMap::new(),
        }
    }
    //TODO: this should be pub?
    /// a function used by the Builder to register a function in the IR
    pub(super) fn register_function(&mut self, block: BlockId, name: String) {
        self.functions.insert(name, block);
    }

    pub fn lookup_function(&self, name: &str) -> Option<BlockId> {
        self.functions.get(name).map(Clone::clone) // BlockIds are trivially cloneable so just clone it to remove the reference
    }

    pub fn get_block(&self, block: BlockId) -> &Block {
        &self.blocks[block.0]
    }
}
