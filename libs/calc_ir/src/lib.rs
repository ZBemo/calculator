pub mod builder;

/// The basic value of any variable in the calculator, a natively sized signed integer
/// You could easily expand this to be a BigInt (arbitrarily sized), or have the IR be able to represent mulitple types,
/// but that might add some complexity
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Number(isize);

/// SAFETY: any register should only be assigned to once
/// A register represents a "pointer" to a Number, Registers should be assigned to once
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Register(usize);

/// A BlockLabel represents a "pointer" to a Block
/// Any BlockLabel should only point to one block
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BlockId(usize);

/// An enum to represent a single Intermediate representation instruction
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    LoadImmediate(Number, Register),

    // block navigation commands
    // call a block with the provided arguments
    Call {
        block: BlockId,
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
    JNoneZero {
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
}
