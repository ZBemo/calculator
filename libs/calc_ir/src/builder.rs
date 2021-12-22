use crate::{BlockId, Operation, Register};

pub type Block = Vec<Operation>;
pub type Program = Vec<Block>;

pub struct BlockBuilder<'a> {
    instructions: Block,
    builder: &'a mut Builder,
}

impl<'a> BlockBuilder<'a> {
    pub fn finalize(self) -> BlockId {
        self.builder.add_block(self.instructions)
    }
}

pub struct Builder {
    used_registers: usize,
    program: Program,
}

impl Builder {
    pub fn finalize(self) -> Program {
        return self.program;
    }

    pub fn new() -> Self {
        Self {
            used_registers: 0,
            program: Vec::new(),
        }
    }

    /// for use with BlockBuilder; adds the block to a list of blocks in the program and returns an identifier unique to that block
    pub(super) fn add_block(&mut self, block: Block) -> BlockId {
        let id = self.program.len(); // the index of the block that we're pushing to the program is the same as the amount of blocks in the program before we add it
        self.program.push(block);
        BlockId(id)
    }

    /// Allocates a register and returns a "pointer" to it, in reality it's just an add and return, as the interpreter is responsible for allocating actual
    /// registers
    pub fn allocate_register(&mut self) -> Register {
        let ret = self.used_registers;
        self.used_registers += 1;
        Register(ret)
    }

    /// Returns a builder for building a Block out of Operations
    pub fn get_block_builder(&mut self) -> BlockBuilder {
        BlockBuilder {
            instructions: Vec::new(),
            builder: self,
        }
    }
}
