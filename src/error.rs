use std::{ fmt::{Debug, Display}};

use crate::Cell;

/// All the errors that can happen
/// Still very basic
/// TODO: document
/// TODO: test
#[derive(PartialEq)]
pub enum Error {
    Compiler(Vec<Cell>,String),
    Executor,
    Parser(String),
    Stack,
    Type,
    Unimplemented(String),
    Prev(Vec<Error>)
}
pub type Result<T> = std::result::Result<T, Error>;
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Compiler(cell, token) => write!(f, "Compilation of Token {:?} failed. F:{:?}", token, cell),
            Error::Executor => write!(f, "Executor"),
            Error::Parser(token) => write!(f, "Parsing failed ({:?})", token),
            Error::Stack => write!(f, "Stack"),
            Error::Type => write!(f, "Type"),
            Error::Unimplemented(name) => write!(f, "Unimplemented({:?})",name),
            Error::Prev(other) => write!(f,"[{:?}]",other),
        }
    }
}