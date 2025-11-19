use crate::{Context, Error, Result, Variable};
use std::{collections::{HashMap, VecDeque}, fmt::{Debug, Display}};

/// interface for rust `word-functions`
///
/// used to write native forth functions in rust
///
/// - `Context`: the forth context to operate on
/// - `VecDeque<&str>`: rest of the input buffer **AFTER** our word was called from forth
///
/// if the forth input looks like this `( this is a comment) some forth Cell`
///
/// `(` will be popped from the input buffer
/// and all following tokens will be passed as `arg2`.
///
type WordFunction = fn(&mut Context, &mut VecDeque<String>) -> Result<()>;
type CompileFunction = fn(&mut Context, &mut VecDeque<String>) -> Result<Cell>;

/// `Cell` represents **named** forth executable ***Cell***
///
/// `Cell` may be either
///
/// - `Native`: rust Cell that will operate on the forth context and input-buffer
/// - `Dynamic`: forth Cell written in forth and *compiled*.
///
#[derive(Debug, Clone)]
pub enum Cell {
    Exec(WordFunction),
    Compiled(CompileFunction),
    Routine(Vec<Cell>),
    Branch(WordFunction, Vec<Cell>),
    Data(Variable),
    Call(String),
    ControlReturn,
    ControlBranch,
}
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            //(Self::Exec(l0), Self::Exec(r0)) => l0 == r0,
            //(Self::Compiled(l0), Self::Compiled(r0)) => l0 == r0,
            (Self::Routine(l0), Self::Routine(r0)) => l0 == r0,
            //(Self::Branch(l0, l1), Self::Branch(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Data(l0), Self::Data(r0)) => l0 == r0,
            (Self::Call(l0), Self::Call(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<WordFunction> for Cell {
    fn from(value: WordFunction) -> Self {
        Cell::Exec(value)
    }
}
impl From<CompileFunction> for Cell {
    fn from(value: CompileFunction) -> Self {
        Cell::Compiled(value)
    }
}
impl From<Vec<Cell>> for Cell {
    fn from(value: Vec<Cell>) -> Self {
        Cell::Routine(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Dictionary {
    data: HashMap<String, Cell>,
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
        T: Into<Cell>,
    {
        self.data.insert(name.to_string(), dict_value.into());
    }

    pub fn get(&self, name: &str) -> Result<Cell> {
        self.data.get(&name.to_lowercase()).cloned().ok_or(Error::Unimplemented(name.to_owned()))
    }
}
