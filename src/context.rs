use crate::{Cell, Dictionary, Error, Result, Stack, Variable};
use std::{
    any::Any,
    collections::VecDeque,
    fmt::Debug,
    io::{BufRead, Write},
    mem,
};

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Taken,
    Interpret,
    FIllBuffer,
    Compile,
}

impl State {
    pub fn is_idling(&self) -> bool {
        match &self {
            State::Taken => true,
            State::Interpret => true,
            State::FIllBuffer => true,
            State::Compile => false,
        }
    }
}

/// Complete context of the forth env
pub struct VM {
    pub value_stack: Stack,
    pub return_stack: Stack,
    pub dictionary: Dictionary,
    pub write: Box<dyn Fn(&str)>,
    pub read: Box<dyn Fn(&mut String) -> std::io::Result<usize>>,
    pub state: State,
    pub handle_errors: bool,
    pub input_buffer: VecDeque<String>,
}
impl Debug for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VM")
            .field("value_stack", &self.value_stack)
            .field("return_stack", &self.return_stack)
            .field("write", &self.write.type_id())
            .field("read", &self.write.type_id())
            .field("state", &self.state)
            .field("handle_errors", &self.handle_errors)
            .field("input_buffer", &self.input_buffer)
            .finish()
    }
}

impl VM {
    /// Create a new context and bind
    /// input - `stdin`
    /// output - `stdout`
    /// ```no_run
    /// # use frust::VM;
    /// let vm = VM::new_stdio();
    ///
    /// /// read data from stdin
    /// let mut buffer = String::new();
    /// (vm.read)(&mut buffer);
    ///
    /// /// write data to stdout
    /// (vm.write)(&buffer);
    /// ```
    pub fn new_stdio() -> VM {
        Self::new(
            |buf| {
                let stdin = std::io::stdin();
                stdin.lock().read_line(buf)
            },
            |buf| {
                print!("{} ", buf);
                std::io::stdout().flush().unwrap();
            },
        )
    }

    /// Create a new context and bind
    /// input - `null`
    /// output - `null`
    ///
    /// ```
    /// # use frust::VM;
    /// let vm = VM::new_null();
    ///
    /// /// read nothing
    /// let mut buffer = String::new();
    /// (vm.read)(&mut buffer);
    ///
    /// /// write nothing
    /// (vm.write)(&buffer);
    /// ```
    pub fn new_null() -> VM {
        Self::new(|_| Ok(0), |_| {})
    }

    /// Create a new forth VM
    /// `read` - global user input function
    /// `write` - global write to user function
    pub fn new(read: fn(&mut String) -> std::io::Result<usize>, write: fn(&str)) -> VM {
        VM {
            value_stack: Stack::new(),
            return_stack: Stack::new(),
            dictionary: Dictionary::new(),
            write: Box::new(write),
            read: Box::new(read),
            state: State::Interpret,
            handle_errors: true,
            input_buffer: VecDeque::new(),
        }
    }

    // actual "compilation" step
    pub fn compile(&mut self) -> Result<Vec<Cell>> {
        let mut function: Vec<Cell> = Vec::new();
        while let Some(token) = self.input_buffer.pop_front() {
            if token == ";" {
                return Ok(function);
            }
            // if this is a valid word from our dictionary
            // add this to the function to be callable later
            if let Ok(routine) = self.dictionary.get(&token) {
                if routine.len() == 1 {
                    for word in routine {
                        match word {
                            Cell::Compiler(ct_func) => {
                                function.append(&mut ct_func(self)?);
                                break;
                            }
                            _ => function.push(word),
                        }
                    }
                } else {
                    function.push(Cell::Call(token.clone()));
                }
            }
            // try to parse the input as a numeric value
            // this is not std conform we should read `BASE` variable that indicates
            // the radix (2-10-16)
            else if let Ok(value) = std::primitive::i64::from_str_radix(&token, 10) {
                function.push(Cell::Data(Variable::Int(value)).into());
            }
            // unknown token,
            // maybe an error or just a token we are not supposed to compile
            else {
                return Err(Error::Compiler(function, token));
            }
        }
        Err(Error::Compiler(function, "EOL".to_owned()))
    }

    /// transition the compilation state
    fn state_compile(&mut self) -> Result<State> {
        let _ = self.input_buffer.pop_front(); // pop the leading`:`

        if let Some(name) = self.input_buffer.pop_front() {
            let function = self.compile()?;

            self.dictionary.add(&name.to_lowercase(), function);
        }
        Ok(State::Interpret)
    }

    /// executes an entry from the dictionary
    pub fn execute(&mut self, program: Vec<Cell>) -> Result<()> {
        let mut pc = 0i64;
        //println!("execute: {:?}", self);
        //println!("execute: {:?}", program);
        while pc < program.len() as i64 {
            let word = program.get(pc as usize).ok_or(Error::Executor)?.clone();
            //println!("pc({})word({:?})",pc,word);
            //crate::builtins::dot_s(self);

            let mut next_step = 1i64;
            match word {
                Cell::Exec(func) => func(self)?,
                Cell::Compiled(func) => func(self)?,
                Cell::Call(name) => self.execute(self.dictionary.get(&name)?)?,
                Cell::Data(data) => self.value_stack.push(data),
                Cell::ControlReturn => {
                    return Ok(());
                }
                Cell::ControlBranch => {
                    next_step = self.value_stack.pop()?.into();
                }
                Cell::ControlBranchIfZero => {
                    let branch_step: i64 = self.value_stack.pop()?.into();
                    let branch_check = self.value_stack.pop()?;
                    if branch_check == Variable::Int(0) {
                        next_step = branch_step;
                    }
                }
                Cell::ControlBranchIfNotZero => {
                    let branch_step: i64 = self.value_stack.pop()?.into();
                    let branch_check = self.value_stack.pop()?;
                    if branch_check != Variable::Int(0) {
                        next_step = branch_step;
                    }
                }
                Cell::Compiler(_) => {
                    return Err(Error::Parser("Interpreting a compile-only word".to_owned()));
                }
            };
            //crate::builtins::dot_s(self);
            pc += next_step;
        }
        Ok(())
    }

    /// interpret forth tokens
    fn state_interpret(&mut self) -> Result<State> {
        // : indicates start of compilation.
        // we switch to input buffering until we see ;
        // so we can compile the complete function in one go
        if self.input_buffer.front() == Some(&":".to_owned()) {
            return self.state_fill_buffer();
        }

        // interprete all input token by token
        while let Some(token) = self.input_buffer.pop_front() {
            // is this token a word from the dictionary we execute it
            if let Ok(word) = self.dictionary.get(&token) {
                self.execute(word)?;
            }
            // try to parse the input as a numeric value
            // this is not std conform we should read `BASE` variable that indicates
            // the radix (2-10-16)
            else if let Ok(value) = std::primitive::i64::from_str_radix(&token, 10) {
                self.value_stack.push(Variable::Int(value));
            }
            // we don't know how to handle this token
            else {
                return Err(Error::Parser(token));
            }
        }

        Ok(State::Interpret)
    }

    /// stays in fill buffer state until it sees a ';'
    fn state_fill_buffer(&mut self) -> Result<State> {
        if self.input_buffer.contains(&";".to_owned()) {
            return Ok(State::Compile);
        } else {
            return Ok(State::FIllBuffer);
        }
    }

    /// prints error message and resets state machine if wanted
    pub fn state_error(&self, error: Error) -> Result<State> {
        (self.write)(&format!("Error: {}\n", error));
        if self.handle_errors {
            Ok(State::Interpret)
        } else {
            Err(error)
        }
    }

    /// Takes an input and evaluates it.
    /// automatically switch between interpreter and compiler
    /// ```
    /// # use frust::*;
    /// # let mut vm = VM::new_null();
    /// # vm.dictionary.add("+", Cell::Exec(builtins::plus));
    /// # vm.dictionary.add(".", Cell::Exec(builtins::dot));
    /// vm.eval("5 4 + . ");
    /// ```
    pub fn eval(&mut self, input: &str) -> Result<()> {
        self.input_buffer
            .extend(input.split_whitespace().map(|token| token.to_owned()));

        while !self.input_buffer.is_empty() || !self.state.is_idling() {
            let new_state = match mem::take(&mut self.state) {
                State::Taken => Ok(State::Interpret),
                State::Interpret => self.state_interpret(),
                State::FIllBuffer => self.state_fill_buffer(),
                State::Compile => self.state_compile(),
            };
            match new_state {
                Ok(state) => self.state = state,
                Err(error) => self.state = self.state_error(error)?,
            }
        }
        Ok(())
    }
}
