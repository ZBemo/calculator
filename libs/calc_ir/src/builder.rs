//TODO: IMPORTANT:  move register allocation into BlockBuilder, registers shouldn't be constant thoughout a program!
pub mod instructions;

use crate::{Block, BlockId, Instruction, Program, Register};

/// A struct that allows you to build a [`Block`] inside of the context of a [`Builder`]
#[allow(clippy::module_name_repetitions)] // this is the only name I can think of that doesn't conflict
pub struct BlockBuilder<'a> {
    /// The instructions within the block, to be executed in sequential order
    instructions: Block,
    /// The builder that the BlockBuilder is tied to
    builder: &'a mut Builder,

    used_registers: usize,
}

impl<'a> BlockBuilder<'a> {
    /// Allocates a register and returns a "pointer" to it, in reality it's just an add and return, as the interpreter is responsible for allocating actual
    /// registers
    #[must_use = "if you're allocating a register, you probably want to do something with it"]
    fn allocate_register(&mut self) -> Register {
        let ret = self.used_registers;
        self.used_registers += 1;
        Register(ret)
    }

    /// Finalize the Block and register it with the builder, returning an Identifier to that block
    pub fn finalize(self) -> BlockId {
        self.builder.add_block(self.instructions)
    }

    /// Finalizes  self and registers the given blockId as a function with name `fn_name`
    pub fn finalize_as_fn(self, fn_name: String) {
        let id = self.builder.add_block(self.instructions);

        self.builder.register_function(id, fn_name);
    }

    /// Loads the amount of arguments specified in argc into registers, and returns a vector of registers where the first argument
    /// will be loaded into \[0\], the second into \[1\], etc
    pub fn add_loadargs(&mut self, argc: usize) -> Vec<Register> {
        let mut ret = Vec::new();
        for _ in 0..argc {
            ret.push(self.allocate_register());
        }

        self.instructions.push(Instruction::LoadArgs(ret.clone()));

        ret
    }

    pub fn add_immediate(&mut self, imm: super::Number) -> Register {
        let immediate_reg = self.allocate_register();

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
        let out_reg = self.allocate_register();
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
        let out_reg = self.allocate_register();
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
        let out_reg = self.allocate_register();

        self.instructions.push(Instruction::Call {
            name,
            arguments,
            out: out_reg,
        });

        out_reg
    }
}

/// A struct to easily build a program, to add Blocks call [`Builder::build_block`] and use the returned [`BlockBuilder`]
pub struct Builder {
    program: Program,
}

impl Builder {
    /// recreate a builder from a program, allowing you to (only) add functions to it
    pub fn from_program(program: Program) -> Self {
        Self { program }
    }

    pub fn register_function(&mut self, block: BlockId, name: String) {
        self.program.register_function(block, name);
    }

    /// Finalize the program and get a collection of [`Block`]s back, which you can then pass to an interpreter
    pub fn finalize(self) -> Program {
        self.program
    }

    pub fn new() -> Self {
        Self {
            program: Program::new(),
        }
    }

    /// for use from [`BlockBuilder`]. adds the block to a list of blocks in the program and returns an identifier unique to that block
    pub(super) fn add_block(&mut self, block: Block) -> BlockId {
        let id = self.program.blocks.len(); // the index of the block that we're pushing to the program is the same as the amount of blocks in the program before we add it
        self.program.blocks.push(block);
        BlockId(id)
    }

    /// Returns a builder for building a Block out of Operations
    pub fn build_block(&mut self) -> BlockBuilder {
        BlockBuilder {
            instructions: Vec::new(),
            builder: self,
            used_registers: 0,
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
