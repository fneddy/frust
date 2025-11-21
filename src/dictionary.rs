use crate::{Error, Result, VM, Variable};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    os::raw::c_void,
};

/// interface for rust `word-functions`
///
/// used to write native forth functions in rust
///
/// - `VM`: the forth context to operate on
/// - `VecDeque<&str>`: rest of the input buffer **AFTER** our word was called from forth
///
/// if the forth input looks like this `( this is a comment) some forth Cell`
///
/// `(` will be popped from the input buffer
/// and all following tokens will be passed as `arg2`.
///
type WordFunction = fn(&mut VM) -> Result<()>;
type CompileFunction = fn(&mut VM) -> Result<Vec<Cell>>;

/// `Cell` represents **named** forth executable ***Cell***
///
/// `Cell` may be either
///
/// - `Native`: rust Cell that will operate on the forth context and input-buffer
/// - `Dynamic`: forth Cell written in forth and *compiled*.
///
#[derive(PartialEq, Debug, Clone)]
pub enum Cell {
    Exec(WordFunction),
    Compiler(CompileFunction),
    Compiled(WordFunction),
    Data(Variable),
    Call(String),
    ControlReturn,
    ControlBranch,
    ControlBranchIfZero,
    ControlBranchIfNotZero,
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<Cell> for Vec<Cell> {
    fn from(value: Cell) -> Self {
        vec![value]
    }
}
impl From<WordFunction> for Cell {
    fn from(value: WordFunction) -> Self {
        Cell::Exec(value)
    }
}
impl From<CompileFunction> for Cell {
    fn from(value: CompileFunction) -> Self {
        Cell::Compiler(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Dictionary {
    data: HashMap<String, Vec<Cell>>,
}

impl Dictionary {
    /// create a new dictionary
    ///
    /// ```
    /// # use frust::Dictionary;
    /// let dict = Dictionary::new();
    /// ```
    pub fn new() -> Dictionary {
        Dictionary {
            data: HashMap::new(),
        }
    }

    pub fn add<T>(&mut self, name: &str, dict_value: T)
    where
        T: Into<Vec<Cell>>,
    {
        self.data.insert(name.to_string(), dict_value.into());
    }

    pub fn get(&self, name: &str) -> Result<Vec<Cell>> {
        self.data
            .get(&name.to_lowercase())
            .cloned()
            .ok_or(Error::Unimplemented(name.to_owned()))
    }
}
