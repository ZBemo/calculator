//! Structs to build a valid `zach_ir` program this is just one implementation of a builder,
//! and is nowhere near the most efficient, in terms of code generation, but it is extremely simple and safe

pub mod instructions;

use std::collections::HashMap;

use crate::program::implementations::*;
use crate::Register;

/// a "real" instruction type as opposed to the generic type
type Instruction = crate::Instruction<BlockID, String>;
/// this is just easier to read in my opinion
type IRBlock = Vec<Instruction>;

pub struct Block<'a, 'b> {
    instructions: Vec<Instruction>,
    function: &'a mut Function<'b>,
}

impl<'a, 'b> Block<'a, 'b> {
    #[must_use = "If you're creating a Block, it's useless not to use it and will be destroyed during optimization regardless"]
    pub fn finalize(self) -> BlockID {
        self.function.program.register_block(self.instructions)
    }

    /// Add a function call to the block's instructions, returning its output register
    pub fn add_fn_call(&mut self, name: String, arguments: Vec<Register>) -> Register {
        let ret_reg = self.function.allocate_register();
        self.instructions.push(Instruction::Call {
            function_id: name,
            arguments,
            out: ret_reg,
        });
        ret_reg
    }

    /// Add a jump to another, already existing, Block
    ///
    /// In the future we might provide a way to jump to a block that is created in the future, but right now you need to build functions "from the bottom up".
    pub fn add_cond_jump(&mut self, jump_type: instructions::BlockJump, to: BlockID) {
        match jump_type {
            instructions::BlockJump::Unconditional => self.instructions.push(Instruction::Jump(to)),
            instructions::BlockJump::Equal(lhs, rhs) => {
                self.instructions.push(Instruction::JEqual { to, lhs, rhs });
            }
            instructions::BlockJump::NotEqual(lhs, rhs) => self
                .instructions
                .push(Instruction::JNotEqual { to, lhs, rhs }),
            instructions::BlockJump::NoneZero(r) => self
                .instructions
                .push(Instruction::JNonZero { check: r, to }),
            instructions::BlockJump::Zero(r) => {
                self.instructions.push(Instruction::JZero { check: r, to });
            }
        }
    }

    pub fn add_arithmetic(
        &mut self,
        operation: instructions::Arithmetic,
        lhs: Register,
        rhs: Register,
    ) -> Register {
        let out = self.function.allocate_register();

        match operation {
            instructions::Arithmetic::Add => {
                self.instructions.push(Instruction::Add { out, lhs, rhs });
            }
            instructions::Arithmetic::Subtract => {
                self.instructions
                    .push(Instruction::Subtract { out, lhs, rhs });
            }
            instructions::Arithmetic::Multiply => {
                self.instructions
                    .push(Instruction::Multiply { out, lhs, rhs });
            }
            instructions::Arithmetic::Divide => {
                self.instructions
                    .push(Instruction::Divide { out, lhs, rhs });
            }
            instructions::Arithmetic::Mod => {
                self.instructions
                    .push(Instruction::Modulo { out, lhs, rhs });
            }
        }

        out
    }

    /// Add a bitwise instruction
    pub fn add_bitwise(
        &mut self,
        operation: instructions::BitWise,
        lhs: Register,
        rhs: Register,
    ) -> Register {
        let out = self.function.allocate_register();

        match operation {
            instructions::BitWise::Or => {
                self.instructions.push(Instruction::BitOr { out, lhs, rhs });
            }
            instructions::BitWise::NotOr => {
                self.instructions
                    .push(Instruction::BitNotOr { out, lhs, rhs });
            }
            instructions::BitWise::And => {
                self.instructions
                    .push(Instruction::BitAnd { out, lhs, rhs });
            }
            instructions::BitWise::ShiftLeft => {
                self.instructions
                    .push(Instruction::ShiftL { out, lhs, rhs });
            }
            instructions::BitWise::ShiftRight => {
                self.instructions
                    .push(Instruction::ShiftR { out, lhs, rhs });
            }
        }

        out
    }
}

pub struct Function<'a> {
    used_registers: usize,
    pub(self) program: &'a mut Program,
    name: String,
}

impl<'a> Function<'a> {
    /// create a new function, with its own enclosing "scope"
    pub(self) fn new(name: String, program: &'a mut Program) -> Self {
        Self {
            used_registers: 0,
            program,
            name,
        }
    }

    pub(self) fn allocate_register(&mut self) -> Register {
        let ret_reg = Register(self.used_registers);
        self.used_registers += 1;
        ret_reg
    }

    pub fn build_block(&mut self) -> Block<'a, '_> {
        Block {
            instructions: Vec::new(),
            function: self,
        }
    }

    pub fn finalize(self, entry_block: BlockID) {
        self.program.register_function(self.name, entry_block);
    }
}

/// Builds a whole program, probably the first thing you want to get your hands on to start
/// building a [`BasicProgram`]
pub struct Program {
    blocks: Vec<IRBlock>,
    functions: HashMap<String, BlockID>,
}

impl Program {
    pub(self) fn register_block(&mut self, block: IRBlock) -> BlockID {
        let block_id = self.blocks.len();
        self.blocks.push(block);
        BlockID(block_id)
    }

    pub(self) fn register_function(&mut self, name: String, entry: BlockID) {
        self.functions.insert(name, entry);
    }

    #[must_use = "You shouldn't call finalize if you're not ready to use the created Program"]
    pub fn finalize(self) -> BasicProgram {
        BasicProgram {
            function_list: self.functions,
            blocks: self.blocks,
        }
    }

    pub fn make_fn(&mut self, function_name: String) -> Function {
        Function::new(function_name, self)
    }
}
