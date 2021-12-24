pub mod instructions;

use crate::{BlockId, Instruction, Register};
use std::collections::HashMap;

/// A "block" of code, useful for loops, if commands, etc
pub type Block = Vec<Instruction>;

///TODO:  this probably doesn't belong in builder, move to lib.rs
/// a collection of Blocks and functions, built by a `Builder`
pub struct Program {
    blocks: Vec<Block>,
    functions: HashMap<String, BlockId>,
}

impl Program {
    fn new() -> Self {
        Self {
            blocks: Vec::new(),
            functions: HashMap::new(),
        }
    }
    //TODO: this should be pub?
    /// a function used by the Builder to register a function in the IR
    fn register_function(&mut self, block: BlockId, name: String) {
        self.functions.insert(name, block);
    }

    pub fn lookup_function(&self, name: &str) -> Option<BlockId> {
        self.functions.get(name).map(Clone::clone) // BlockIds are trivially cloneable so just clone it to remove the reference
    }

    pub fn get_block(&self, block: BlockId) -> &Block {
        &self.blocks[block.0]
    }
}

/// A struct that allows you to build a `Block` inside of the context of a `Builder`
#[allow(clippy::module_name_repetitions)] // this is the only name I can think of that doesn't conflict
pub struct BlockBuilder<'a> {
    /// The instructions within the block, to be executed in sequential order
    instructions: Block,
    /// The builder that the BlockBuilder is tied to
    builder: &'a mut Builder,
}

impl<'a> BlockBuilder<'a> {
    /// Finalize the Block and register it with the builder, returning an Identifier to that block
    pub fn finalize(self) -> BlockId {
        self.builder.add_block(self.instructions)
    }

    pub fn add_immediate(&mut self, imm: super::Number) -> Register {
        let immediate_reg = self.builder.allocate_register();

        self.instructions
            .push(Instruction::LoadImmediate(imm, immediate_reg));

        immediate_reg
    }

    pub fn add_ret(&mut self, reg: Register) {
        self.instructions.push(Instruction::Ret(reg));
    }

    /// Adds an Arithmetic operation, and returns the register that it will store its result in
    pub fn add_arithmetic(
        &mut self,
        operation: instructions::Arithmetic,
        lhs: Register,
        rhs: Register,
    ) -> Register {
        let out_reg = self.builder.allocate_register();
        match operation {
            instructions::Arithmetic::Add => {
                self.instructions.push(Instruction::Add {
                    lhs,
                    rhs,
                    out: out_reg,
                });
            }
            instructions::Arithmetic::Subtract => {
                self.instructions.push(Instruction::Subtract {
                    lhs,
                    rhs,
                    out: out_reg,
                });
            }
            instructions::Arithmetic::Multiply => {
                self.instructions.push(Instruction::Multiply {
                    lhs,
                    rhs,
                    out: out_reg,
                });
            }
            instructions::Arithmetic::Divide => {
                self.instructions.push(Instruction::Divide {
                    lhs,
                    rhs,
                    out: out_reg,
                });
            }
            instructions::Arithmetic::Mod => {
                self.instructions.push(Instruction::Modulo {
                    lhs,
                    rhs,
                    out: out_reg,
                });
            }
        };
        out_reg
    }

    pub fn add_bitwise(
        &mut self,
        operation: instructions::BitWise,
        lhs: Register,
        rhs: Register,
    ) -> Register {
        let out_reg = self.builder.allocate_register();
        match operation {
            instructions::BitWise::Or => self.instructions.push(Instruction::BitOr {
                lhs,
                rhs,
                out: out_reg,
            }),

            instructions::BitWise::NotOr => self.instructions.push(Instruction::BitNotOr {
                lhs,
                rhs,
                out: out_reg,
            }),
            instructions::BitWise::And => self.instructions.push(Instruction::BitAnd {
                lhs,
                rhs,
                out: out_reg,
            }),
            instructions::BitWise::ShiftLeft => self.instructions.push(Instruction::ShiftL {
                lhs,
                rhs,
                out: out_reg,
            }),
            instructions::BitWise::ShiftRight => self.instructions.push(Instruction::ShiftR {
                lhs,
                rhs,
                out: out_reg,
            }),
        }

        out_reg
    }

    pub fn add_fn_call(&mut self, name: String, arguments: Vec<Register>) -> Register {
        let out_reg = self.builder.allocate_register();

        self.instructions.push(Instruction::Call {
            name,
            arguments,
            out: out_reg,
        });

        out_reg
    }
}

/// A struct to easily build a program, to add Blocks call `build_block` and use the returned `BlockBuilder`
pub struct Builder {
    used_registers: usize,
    program: Program,
}

impl Builder {
    /// recreate a builder from a program, as well as the `previous_used_registers` which you can obtain by calling `used_registers` before finalize
    pub fn from_program(program: Program, previous_used_registers: usize) -> Self {
        Self {
            used_registers: previous_used_registers,
            program,
        }
    }

    /// This should be called before finalize() if you plan to re-build a Builder from the returned program at a later state in time
    pub fn used_registers(&self) -> usize {
        self.used_registers
    }

    pub fn register_function(&mut self, block: BlockId, name: String) {
        self.program.register_function(block, name);
    }

    /// Finalize the program and get a collection of `Block`s back, which you can then pass to an interpreter
    pub fn finalize(self) -> Program {
        self.program
    }

    pub fn new() -> Self {
        Self {
            used_registers: 0,
            program: Program::new(),
        }
    }

    /// for use from `BlockBuilder`. adds the block to a list of blocks in the program and returns an identifier unique to that block
    pub(super) fn add_block(&mut self, block: Block) -> BlockId {
        let id = self.program.blocks.len(); // the index of the block that we're pushing to the program is the same as the amount of blocks in the program before we add it
        self.program.blocks.push(block);
        BlockId(id)
    }

    /// Allocates a register and returns a "pointer" to it, in reality it's just an add and return, as the interpreter is responsible for allocating actual
    /// registers
    #[must_use = "if you're allocating a register, you probably want to do something with it"]
    pub fn allocate_register(&mut self) -> Register {
        let ret = self.used_registers;
        self.used_registers += 1;
        Register(ret)
    }

    /// Returns a builder for building a Block out of Operations
    pub fn build_block(&mut self) -> BlockBuilder {
        BlockBuilder {
            instructions: Vec::new(),
            builder: self,
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
