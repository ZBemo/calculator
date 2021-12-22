pub mod builder;

/// The basic value of any variable in the calculator, a natively sized signed integer
#[derive(Debug, PartialEq, Eq)]
pub struct Number(isize);

/// SAFETY: any register should only be assigned to once
/// A register represents a "pointer" to a Number, Registers should be assigned to once
#[derive(Debug, PartialEq, Eq)]
pub struct Register(usize);

/// A BlockLabel represents a "pointer" to a Block
/// Any BlockLabel should only point to one block
#[derive(Debug, PartialEq, Eq)]
pub struct BlockId(usize);

/// An enum to represent a single Intermediate representation instruction
#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    LoadImmediate(Number, Register),

    // block navigation commands
    Call {
        block: BlockId,
        arguments: Vec<Register>,
        //TODO: out should be a vec of registers?
        out: Register,
    },
    // jump back to caller
    Ret,
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
    BitXor {
        lhs: Register,
        rhs: Register,
        out: Register,
    },
    BitAnd {
        lhs: Register,
        rhs: Register,
        out: Register,
    },
    BitNot {
        input: Register,
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
