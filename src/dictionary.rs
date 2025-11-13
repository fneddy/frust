use crate::{Context, Result, Variable};
use std::collections::{HashMap, VecDeque};

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
#[derive(Debug, PartialEq, Clone)]
pub enum Cell {
    Call(WordFunction),
    Compiled(WordFunction, CompileFunction),
    Routine(Vec<Cell>),
    Branch(WordFunction, Vec<Cell>),
    Label(String),
    Data(Variable),
}
impl From<WordFunction> for Cell {
    fn from(value: WordFunction) -> Self {
        Cell::Call(value)
    }
}
impl From<(WordFunction, CompileFunction)> for Cell {
    fn from(value: (WordFunction, CompileFunction)) -> Self {
        Cell::Compiled(value.0, value.1)
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

    pub fn get(&self, name: &str) -> Option<Cell> {
        self.data.get(&name.to_lowercase()).cloned()
    }
}
