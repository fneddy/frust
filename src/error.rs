use std::fmt::Display;

/// All the errors that can happen
/// Still very basic
/// TODO: document
/// TODO: test
#[derive(Debug, PartialEq)]
pub enum Error {
    Compiler,
    Executor,
    Parser,
    Stack,
    Type,
    Unimplemented,
}
pub type Result<T> = std::result::Result<T, Error>;
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
