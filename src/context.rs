use crate::{builtins::unimplemented, Dictionary, Error, Result, Stack, Variable};
use std::{collections::VecDeque, io::BufRead};

#[derive(Debug, Default)]
enum State {
    #[default]
    Interpret,
    Compile(String),
}

/// Complete context of the forth env
#[derive(Debug)]
pub struct Context {
    pub value_stack: Stack,
    pub return_stack: Stack,
    pub dictionary: Dictionary,
    pub write: fn(&str),
    pub read: fn(&mut String) -> std::io::Result<usize>,
    state: State,
}

impl Context {
    /// Create a new context and bind
    /// input - `stdin`
    /// output - `stdout`
    /// ```no_run
    /// # use frust::Context;
    /// let ctx = Context::new_stdio();
    ///
    /// /// read data from stdin
    /// let mut buffer = String::new();
    /// (ctx.read)(&mut buffer);
    ///
    /// /// write data to stdout
    /// (ctx.write)(&buffer);
    /// ```
    pub fn new_stdio() -> Context {
        Self::new(
            |buf| {
                let stdin = std::io::stdin();
                stdin.lock().read_line(buf)
            },
            |buf| {
                println!("{}", buf);
            },
        )
    }

    /// Create a new context and bind
    /// input - `null`
    /// output - `null`
    ///
    /// ```
    /// # use frust::Context;
    /// let ctx = Context::new_null();
    ///
    /// /// read nothing
    /// let mut buffer = String::new();
    /// (ctx.read)(&mut buffer);
    ///
    /// /// write nothing
    /// (ctx.write)(&buffer);
    /// ```
    pub fn new_null() -> Context {
        Self::new(|_| Ok(0), |_| {})
    }

    /// Create a new forth Context
    /// `read` - global user input function
    /// `write` - global write to user function
    pub fn new(read: fn(&mut String) -> std::io::Result<usize>, write: fn(&str)) -> Context {
        Context {
            value_stack: Stack::new(),
            return_stack: Stack::new(),
            dictionary: Dictionary::new(),
            write,
            read,
            state: State::Interpret,
        }
    }

    /// Split the input into tokens by forth rules
    /// ```ignore
    /// # use frust::Context;
    /// let input = " This is an example for some_forth input";
    /// let tokens = Context::tokenize(input);
    /// ```
    fn tokenize(input: &str) -> VecDeque<&str> {
        input.split_whitespace().collect()
    }

    /// interpret forth tokens
    /// if this encounter a : it will switch to compilation mode
    fn interpret(&mut self, tokens: &mut VecDeque<&str>) -> Result<()> {
        while let Some(token) = tokens.pop_front() {
            // : indicates start of compilation mode
            // we switch internal state to compilation
            // and hand over input tokens to compiler
            if token == ":" {
                if let Some(name) = tokens.pop_front() {
                    self.state = State::Compile(name.into());
                    return self.compile(tokens);
                } else {
                    // compilation needs a function name
                    return Err(Error::Parser);
                }
            // is this token a word from the dictionary
            } else if let Some(word) = self.dictionary.get(token) {
                let _ = match word.code {
                    // static word pointing to rust code
                    Some(crate::Code::Native(function)) => {
                        let _ = function(self, tokens)?;
                    }
                    // dynamic word pointing to runtime compiled code
                    Some(crate::Code::Dynamic(_)) => {}
                    _ => return Err(Error::Parser),
                };

            // try to parse the input as a numeric value
            // this is not std conform we should read `BASE` variable that indicates
            // the radix (2-10-16)
            } else if let Ok(value) = std::primitive::i64::from_str_radix(token, 10) {
                self.value_stack.push(Variable::Int(value));

            // we don't know how to handle this token
            } else {
                (self.write)(&format!("{} not valid", token));
                return Err(Error::Parser);
            }
        }

        Ok(())
    }

    fn compile(&mut self, tokens: &mut VecDeque<&str>) -> Result<()> {
        while let Some(token) = tokens.pop_front() {
            // ; indicates end of compilation
            // we switch over to interpreter mode
            // and hand over the rest of the input
            if token == ";" {
                self.state = State::Interpret;
                return self.interpret(tokens);
            }
        }
        Err(Error::Unimplemented)
    }

    /// Takes an input and evaluates it.
    /// automatically switch between interpreter and compiler
    /// ```
    /// # use frust::*;
    /// # let mut ctx = Context::new_null();
    /// # ctx.dictionary.add("+", Code::Native(builtins::plus));
    /// # ctx.dictionary.add(".", Code::Native(builtins::dot));
    /// ctx.eval("5 4 + . ");
    /// ```
    pub fn eval(&mut self, input: &str) -> Result<()> {
        match self.state {
            State::Interpret => self.interpret(&mut Self::tokenize(input)),
            State::Compile(_) => self.compile(&mut Self::tokenize(input)),
        }
    }
}
