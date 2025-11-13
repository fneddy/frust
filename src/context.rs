use crate::{Cell,  Dictionary, Error, Result, Stack, Variable};
use std::{collections::VecDeque, io::BufRead, mem};

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Taken,
    Interpret,
    FIllBuffer(VecDeque<String>),
    Compile(VecDeque<String>),
}

impl State {
    pub fn is_idling(&self) -> bool {
        match &self {
            State::Taken => true,
            State::Interpret => true,
            State::FIllBuffer(_) => true,
            State::Compile(_) => false,
        }
    }
}

/// Complete context of the forth env
pub struct Context {
    pub value_stack: Stack,
    pub return_stack: Stack,
    pub dictionary: Dictionary,
    pub write: Box<dyn Fn(&str)>,
    pub read: Box<dyn Fn(&mut String) -> std::io::Result<usize>>,
    pub state: State,
    pub handle_errors: bool,
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
            write: Box::new(write),
            read: Box::new(read),
            state: State::Interpret,
            handle_errors: true,
        }
    }

    /// Split the input into tokens by forth rules
    /// ```ignore
    /// # use frust::Context;
    /// let input = " This is an example for some_forth input";
    /// let tokens = Context::tokenize(input);
    /// assert_eq!("This", tokens.pop_front());
    /// ```
    fn tokenize(input: &str) -> VecDeque<String> {
        input
            .split_whitespace()
            .map(|token| token.to_owned())
            .collect()
    }

    /// executes an entry from the dictionary
    fn execute(&mut self, function: Cell, tokens: &mut VecDeque<String>) -> Result<()> {
        
        match function {
            Cell::Call(function) => function(self, tokens),
            Cell::Routine(functions) => functions.iter().map(|function| self.execute(function.clone(), tokens)).collect(),
            Cell::Compiled(rt_function,_) => rt_function(self,tokens),
            Cell::Label(_) => Ok(()),
            Cell::Branch(_,_) => Ok(()),
            Cell::Data(data) => { self.value_stack.push(data); Ok(())},
        }
    }

    /// interpret forth tokens
    fn state_interpret(&mut self, tokens: &mut VecDeque<String>) -> Result<State> {
        // : indicates start of compilation.
        // we switch to input buffering until we see ;
        // so we can compile the complete function in one go
        if tokens.front() == Some(&":".to_owned()) {
            return self.state_fill_buffer(VecDeque::new(), tokens);
        }

        // interprete all input token by token
        while let Some(token) = tokens.pop_front() {
            // is this token a word from the dictionary we execute it
            if let Some(word) = self.dictionary.get(&token) {
                if let Err(error) = self.execute(word, tokens) {
                    return Err(error);
                }
            }
            // try to parse the input as a numeric value
            // this is not std conform we should read `BASE` variable that indicates
            // the radix (2-10-16)
            else if let Ok(value) = std::primitive::i64::from_str_radix(&token, 10) {
                self.value_stack.push(Variable::Int(value));
            }
            // we don't know how to handle this token
            else {
                (self.write)(&format!("{} not valid", token));
                return Err(Error::Parser);
            }
        }

        Ok(State::Interpret)
    }

    // actual "compilation" step
    fn compile(&mut self, tokens: &mut VecDeque<String>) -> Result<Cell> {
        let mut function: Vec<Cell> = Vec::new();
        while let Some(token) = tokens.pop_front() {
            // if this is a valid word from our dictionary
            // add this to the function to be callable later
            if let Some(word) = self.dictionary.get(&token) {
                match word {
                    Cell::Compiled(_,ct_func) => function.push(ct_func(self,tokens)?),
                    _ => function.push(word),
                }
            }
            // try to parse the input as a numeric value
            // this is not std conform we should read `BASE` variable that indicates
            // the radix (2-10-16)
            else if let Ok(value) = std::primitive::i64::from_str_radix(&token, 10) {
                function.push(Cell::Data(Variable::Int(value)).into());
            }
        }
        Ok(Cell::Routine(function))
    }

    /// transition the compilation state
    fn state_compile(&mut self, mut tokens: VecDeque<String>) -> Result<State> {
        let _ = tokens.pop_front(); // pop the leading`:`

        if let Some(name) = tokens.pop_front() {
            let function = self.compile(&mut tokens)?;
            
            self.dictionary.add(&name, function);
        }
        Ok(State::Interpret)
    }

    /// stays in fill buffer state until it sees a ';'
    fn state_fill_buffer(
        &mut self,
        mut buffer: VecDeque<String>,
        tokens: &mut VecDeque<String>,
    ) -> Result<State> {
        while let Some(token) = tokens.pop_front() {
            // ; indicates end of compilation
            // we switch over to interpreter mode
            // and hand over the rest of the input
            if token == ";" {
                return Ok(State::Compile(buffer));
            }
            // add all other token to the input
            else {
                buffer.push_back(token.to_owned());
            }
        }

        return Ok(State::FIllBuffer(buffer));
    }

    /// prints error message and resets state machine if wanted
    pub fn state_error(&self, error: Error) -> Result<State> {
        (self.write)(&format!("{:#?}", error));
        if self.handle_errors {
            Ok(State::Interpret)
        }
        else {
            Err(error)
        }
    }

    /// Takes an input and evaluates it.
    /// automatically switch between interpreter and compiler
    /// ```
    /// # use frust::*;
    /// # let mut ctx = Context::new_null();
    /// # ctx.dictionary.add("+", Cell::Call(builtins::plus));
    /// # ctx.dictionary.add(".", Cell::Call(builtins::dot));
    /// ctx.eval("5 4 + . ");
    /// ```
    pub fn eval(&mut self, input: &str) -> Result<()> {
        let tokens = &mut Self::tokenize(input);

        while !tokens.is_empty() || !self.state.is_idling() {

            let new_state = match mem::take(&mut self.state) {
                State::Taken => Ok(State::Interpret),
                State::Interpret => self.state_interpret(tokens),
                State::FIllBuffer(buffer) => self.state_fill_buffer(buffer, tokens),
                State::Compile(buffer) => self.state_compile(buffer)
            };
            match new_state {
                Ok(state) => self.state = state,
                Err(error) => { self.state = self.state_error(error)? }
            }
        }
        Ok(())
    }
}
