use std::collections::VecDeque;

use crate::*;

/// forth `if` command runtime evaluation
/// 
/// https://forth-standard.org/standard/core/IF
/// 
/// 
/// ```
/// # use std::collections::VecDeque;
/// # use frust::*;
/// # use frust::builtins::eq;
/// # let mut empty: VecDeque<String>  = vec![].into();
/// let mut ctx = Context::new_null();
/// 
/// ```
/// 
pub fn runtime_if(ctx: &mut Context, _: &mut VecDeque<String>) -> Result<()> {
    todo!()
}

/// forth `if` command compiletime evaluation
/// 
/// https://forth-standard.org/standard/core/IF
/// 
/// 
/// ```
/// # use std::collections::VecDeque;
/// # use frust::*;
/// # use frust::builtins::eq;
/// # let mut empty: VecDeque<String>  = vec![].into();
/// let mut ctx = Context::new_null();
/// 
/// ```
/// 
pub fn compiletime_if(ctx: &mut Context, _: &mut VecDeque<String>) -> Result<Cell> {
    Ok(Cell::Branch(runtime_if, vec![]))
}


pub fn runtime_dot_q(ctx: &mut Context, _: &mut VecDeque<String>) -> Result<()> {
    let comment = ctx.value_stack.pop()?;
    (ctx.write)(&format!("{}",comment));
    Ok(())
}

/// forth `."` command compiletime evaluation
/// 
/// https://forth-standard.org/standard/core/Dotq
/// 
/// collect everything between ." " into a string
/// and save this string in the function.
/// at runtime pop this string and print it out.
/// 
/// ```
/// # use frust::*;
/// # use std::sync::mpsc::channel;
/// # let (test_writer, test_stdout) = channel();
/// let mut ctx = Context::new_null();
/// # ctx.write = Box::new( move |str: &str|  {test_writer.send(str.to_owned());});
/// 
/// ctx.dictionary.add(".\"",Cell::Compiled(builtins::runtime_dot_q, builtins::compiletime_dot_q));
/// ctx.dictionary.add(".", Cell::Call(builtins::dot));
/// ctx.dictionary.add("+", Cell::Call(builtins::plus));
/// 
/// ctx.eval(": foo .\" bar baz \" 1 1 + ; ");
/// ctx.eval(" foo . ");
/// assert_eq!(test_stdout.recv().unwrap(), "bar baz");
/// assert_eq!(test_stdout.recv().unwrap(), "2");
/// ```
pub fn compiletime_dot_q(ctx: &mut Context, tokens: &mut VecDeque<String>) -> Result<Cell> {
    let mut buffer = String::new();
    while let Some(token) = tokens.pop_front() {
        
        if token == "\"" {
            let comment = Cell::Data(Variable::from(buffer.as_str()));
            let entry = Cell::Call(runtime_dot_q);
            return Ok(Cell::Routine(vec![comment, entry]));
        }
        if buffer.len() > 0 {
            buffer.push_str(" ");
        }
        buffer.push_str(&token);
    }
    return Err(Error::Parser);
}
