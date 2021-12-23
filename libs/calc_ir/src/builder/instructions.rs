//! the instruction options for BlockBuilder.add_* functions

use crate::Register;
/// An instruction that performs mathematical operations on two `super::Number`s
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitWise {
    Or,
    NotOr,
    And,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Jump {
    Unconditional,
    Equal(Register, Register),
    NotEqual(Register, Register),
    NoneZero(Register),
    Zero(Register),
}
