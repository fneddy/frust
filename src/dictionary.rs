use crate::{Context, Result, Variable};
use std::collections::{HashMap, VecDeque};

/// `Data` represents **named** forth ***Data***. 
/// 
/// `Data` may be either:
/// 
/// `Constant` - can not be changed
/// `Var` - can be changed
/// `Array` - multiple `Variable` instances
///
/// TODO are arrays always variables?
///
/// its possible to crate Data from a Variable
/// ```
/// # use frust::Variable;
/// # use frust::Data;
/// let x: Variable = 1.into();
/// let y: Data = x.into();
/// assert_eq!(y, Data::Var(Variable::Int(1)))
/// ```
///
/// also ts possible to crate Data from a Vec<Variable>
/// ```
/// # use frust::Variable;
/// # use frust::Data;
/// let x: Vec<Variable> = vec![1.into(), 2.into(), 3.into()];
/// let y: Data = x.into();
/// assert_eq!(y, Data::Array(
/// 		vec![Variable::Int(1),Variable::Int(2),Variable::Int(3)]
/// ))
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    Var(Variable),
    Constant(Variable),
    Array(Vec<Variable>),
}

/// Create a `Data` from a Variable
impl From<Variable> for Data {
    fn from(value: Variable) -> Self {
        Data::Var(value)
    }
}

/// Create a `Data-Array` from a `Vec<Variable>`
impl From<Vec<Variable>> for Data {
    fn from(value: Vec<Variable>) -> Self {
        Data::Array(value)
    }
}

/// interface for rust `word-functions`
/// 
/// used to write native forth functions in rust
/// 
/// - `Context`: the forth context to operate on
/// - `VecDeque<&str>`: rest of the input buffer **AFTER** our word was called from forth
/// 
/// if the forth input looks like this `( this is a comment) some forth code`
/// 
/// `(` will be popped from the input buffer
/// and all following tokens will be passed as `arg2`.
/// 
type WordFunction = fn(&mut Context, &mut VecDeque<String>) -> Result<()>;
type CompileFunction = fn(&mut Context, &mut VecDeque<String>) -> Result<()>;

/// `Code` represents **named** forth executable ***Code***
/// 
/// `Code` may be either
/// 
/// - `Native`: rust code that will operate on the forth context and input-buffer
/// - `Dynamic`: forth code written in forth and *compiled*.
/// 
#[derive(Debug, PartialEq, Clone)]
pub enum Code {
    Call(WordFunction),
    Compiled(WordFunction,CompileFunction),
    Routine(Vec<DictionaryEntry>),
    Branch(Vec<Code>),
    Label(String),
}
impl From<WordFunction> for Code {
    fn from(value: WordFunction) -> Self {
        Code::Call(value)
    }
}
impl From<(WordFunction,CompileFunction)> for Code {
    fn from(value: (WordFunction,CompileFunction)) -> Self {
        Code::Compiled(value.0, value.1)
    }
}
impl From<Vec<DictionaryEntry>> for Code {
    fn from(value: Vec<DictionaryEntry>) -> Self {
        Code::Routine(value)
    }
}

///
#[derive(Debug, PartialEq, Clone)]
pub struct DictionaryEntry {
    pub code: Option<Code>,
    pub data: Option<Data>,
}
impl From<Code> for DictionaryEntry {
    fn from(value: Code) -> Self {
        DictionaryEntry {
            code: Some(value),
            data: None,
        }
    }
}
impl From<Data> for DictionaryEntry {
    fn from(value: Data) -> Self {
        DictionaryEntry {
            code: None,
            data: Some(value),
        }
    }
}
impl From<(Code, Data)> for DictionaryEntry {
    fn from(value: (Code, Data)) -> Self {
        DictionaryEntry {
            code: Some(value.0),
            data: Some(value.1),
        }
    }
}

//                                                             
//                      ┌────────────┐                         
//                      │ Dictionary │                         
//                      │   ┌─────┐  │                         
//                      │   │ Name│  │                         
//                      └───└──┬──┘──┘                         
//                             │                               
//                             ▼                               
//                     ┌───────────────┐                       
//                     │DictionaryEntry│                       
//                     │               │                       
//                     │code<Code>─────┼──┐                    
//   ┌─────────────────┤data<Data>     │  │                    
//   │                 └───────────────┘  │                    
//   ▼                                    ▼                    
// ┌────────────────────┐              ┌────────────────────┐  
// │Data                │              │Code                │  
// │                    │              │                    │  
// │Var<Variable>       │              │Native<WordFunction>│  
// │Const<Variable>     │              │Dynamic<Vec<DictionaryEntry>>  │  
// │Array<Vec<Variable>>│              └────────────────────┘  
// └────────────────────┘                                      
//                                                             
#[derive(Debug, PartialEq)]
pub struct Dictionary {
    data: HashMap<String, DictionaryEntry>,
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
        T: Into<DictionaryEntry>,
    {
        self.data.insert(name.to_string(), dict_value.into());
    }

    pub fn get(&self, name: &str) -> Option<DictionaryEntry> {
        self.data.get(&name.to_lowercase()).cloned()
    }
}
