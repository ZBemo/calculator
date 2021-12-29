// clippy configuration
#![warn(clippy::pedantic, clippy::all, clippy::perf)]

pub mod builder;
pub mod program;
pub use program::{BlockID, Program};

/// The basic value of any variable in the calculator, a natively sized signed integer
/// You could easily expand this to be an arbitrarily sized integer, or have the IR be able to represent mulitple types,
/// but that might add some complexity
///
/// This isn't a newtype because newtypes are ANNOYING AS HELL when you want them to just have the exact same semantics as the underlying type.
/// in a real IR you'd want the contents of registers to be newtypes with their own IR specific semantics, but that's unnecassary to implement a simple calculator.
pub type Number = isize;

/// SAFETY: any register should only be assigned to once
/// A register represents a "pointer" to a Number, Registers should be assigned to once
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Register(pub usize);

/// An enum to represent a single Intermediate representation instruction
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction<BlockId: Eq, FunctionId: Eq> {
    LoadImmediate(Number, Register),

    // block navigation commands
    // call a block with the provided arguments
    Call {
        function_id: FunctionId,
        arguments: Vec<Register>,
        // the register to store the return value in
        // this is an inefficient return scheme, architect a better one later?
        out: Register,
    },
    // jump back to caller
    // we expect the interpreter to store the expected return register and Copy the value from the register in Ret into it
    Ret(Register),
    // Loads the arguments to the function, as provided by the Call Instruction into the provided registers, in the same order that they were provided
    // Errors if the amount of requested and provided arguments are inequal
    LoadArgs(Vec<Register>),

    // jump to a block unconditionally
    Jump(BlockId),
    // Commands to jump conditionally
    // I might add more later if necesarry
    JEqual {
        lhs: Register,
        rhs: Register,
        to: BlockId,
    },
    JNotEqual {
        lhs: Register,
        rhs: Register,
        to: BlockId,
    },
    JNonZero {
        check: Register,
        to: BlockId,
    },
    JZero {
        check: Register,
        to: BlockId,
    },

    // basic arithmetic
    Add {
        lhs: Register,
        rhs: Register,
        out: Register,
    },
    Subtract {
        lhs: Register,
        rhs: Register,
        out: Register,
    },
    Multiply {
        lhs: Register,
        rhs: Register,
        out: Register,
    },
    Divide {
        lhs: Register,
        rhs: Register,
        out: Register,
    },
    Modulo {
        lhs: Register,
        rhs: Register,
        out: Register,
    },

    // Bit operations, mainly so that we can write useful optimizations
    BitOr {
        lhs: Register,
        rhs: Register,
        out: Register,
    },
    BitNotOr {
        lhs: Register,
        rhs: Register,
        out: Register,
    },
    BitAnd {
        lhs: Register,
        rhs: Register,
        out: Register,
    },
    ShiftL {
        lhs: Register,
        rhs: Register,
        out: Register,
    },
    ShiftR {
        lhs: Register,
        rhs: Register,
        out: Register,
    },

    Invalid,
}
