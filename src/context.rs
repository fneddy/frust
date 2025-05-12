use crate::{Dictionary, Error, Result, Stack, Variable};
use std::{collections::VecDeque, io::BufRead};

/// Complete context of the forth env
#[derive(Debug)]
pub struct Context {
    pub value_stack: Stack,
    pub return_stack: Stack,
    pub dictionary: Dictionary,
    pub write: fn(&str),
    pub read: fn(&mut String) -> std::io::Result<usize>,
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
        }
    }

    /// take an input and evaluate it
    /// ```
    /// # use frust::*;
    /// # let mut ctx = Context::new_null();
    /// # ctx.dictionary.add_code("+", Code::Native(builtins::plus));
    /// # ctx.dictionary.add_code(".", Code::Native(builtins::dot));
    /// ctx.eval("5 4 + . ");
    /// ```
    pub fn eval(&mut self, input: &str) -> Result<()> {
        // split input by whitespace
        let mut tokens: VecDeque<&str> = input.split_whitespace().collect();

        while tokens.len() > 0 {
            if let Some(token) = tokens.pop_front() {
                // is this token a word from the dictionary
                if let Some(word) = self.dictionary.get(token) {
                    let _ = match word.value.code {
                        crate::Code::Native(function) => {
                            let _ = function(self, &mut tokens)?;
                        }
                        _ => return Err(Error::Parser),
                    };

                // this is not std conform we should read `BASE` variable that indicates
                // the radix (2-10-16)
                // is this token an i64?
                } else if let Ok(value) = std::primitive::i64::from_str_radix(token, 10) {
                    self.value_stack.push(Variable::Int(value));

                // we don't know how to handle this token
                } else {
                    (self.write)(&format!("{} not valid", token));
                    return Err(Error::Parser);
                }
            }
        }
        Ok(())
    }
}
