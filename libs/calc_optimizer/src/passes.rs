//! A trait for implementing a pass, as well as a collection of ready-made optimizer passes
//!
//! To get started making a new pass, look at [`OptimizationPass`]

use crate::structs::Graph;
use calc_ir::Program;
use std::fmt::Debug;
use std::hash::Hash;

/// The trait that must be implemented by a struct in order to run an optimization pass, there are example implementations in this module.
pub trait OptimizationPass<FunctionPointerT: Eq + std::fmt::Debug + Clone + Hash> {
    /// The error type returned by this optimization pass if it fails, if it's incapable of failing, then you should consider setting it to
    /// ! or () types
    type Error: std::error::Error;
    /// Optimize a program, returning an Ok(true) if any changes were made to the program, and a [`Self::Error`] if an error occurs
    fn optimize_program(
        &mut self,
        program: &mut Graph<FunctionPointerT>,
    ) -> Result<bool, Self::Error>;
    //TODO: a way to request/require being run after other OptimizationPasses
}

/// A pass run at the end of an optimization pipeline to lower a Graph to a Program
pub trait SolidifyingPass<FunctionPointerT: Eq + std::fmt::Debug + Clone + Hash> {
    /// The error returned by the pass if it fails
    type Error: std::error::Error;
    /// The actual Program that is returned by the pass
    type SolidProgram: Program<FunctionPointer = FunctionPointerT>;

    /// A function that either lowers the graph to a real program, or returns an error describing what went wrong
    fn soldify_program(
        &mut self,
        program: Graph<FunctionPointerT>,
    ) -> Result<Self::SolidProgram, Self::Error>;
}

/// An error type for an optimization pass that should never error
/// It does its best to achieve codegen as if it doesn't exist
pub struct NeverErrors();

#[cfg(debug_assertions)]
impl Debug for NeverErrors {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("An optimization pass has returned a Error while claiming not to error")
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Display for NeverErrors {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("An optimization pass has returned a Error while claiming not to error")
    }
}

#[cfg(not(debug_assertions))]
impl Debug for NeverErrors {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            std::hint::unreachable_unchecked();
        }
    }
}

#[cfg(not(debug_assertions))]
impl std::fmt::Display for NeverErrors {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            std::hint::unreachable_unchecked();
        }
    }
}

impl std::error::Error for NeverErrors {}

/// Eliminates as much dead code as possible, we recommend running this near the beginning of optimization in order to cut out cruft before other passes look at the Program
pub struct DeadCodeElimination();

impl<FunctionPointerT: Eq + std::fmt::Debug + Clone + Hash> OptimizationPass<FunctionPointerT>
    for DeadCodeElimination
{
    type Error = NeverErrors;

    fn optimize_program(
        &mut self,
        program: &mut Graph<FunctionPointerT>,
    ) -> Result<bool, Self::Error> {
        todo!()
    }
}
