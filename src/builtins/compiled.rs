use std::collections::VecDeque;

use crate::*;


/// forth `if` command compiletime evaluation
///
/// https://forth-standard.org/standard/core/IF
///
/// ```
/// # use frust::*;
/// # use std::sync::mpsc::channel;
/// # let (test_writer, test_stdout) = channel();
/// let mut ctx = Context::new_null();
/// # ctx.write = Box::new( move |str: &str|  {test_writer.send(str.to_owned());});
///
/// ctx.dictionary.add("if",Cell::Compiled(builtins::compiletime_if));
/// ctx.dictionary.add(".\"",Cell::Compiled(builtins::compiletime_dot_q));
/// ctx.dictionary.add(".", Cell::Exec(builtins::dot));
///
/// ctx.eval(": foo IF . ELSE .\" No more \" THEN ; ");
/// ctx.eval(" 1 foo foo ");
/// assert_eq!(test_stdout.recv().unwrap(), "1 No more");
/// ```
pub fn compiletime_if(ctx: &mut Context, tokens: &mut VecDeque<String>) -> Result<Cell> {
    let mut branches = Vec::new();

    while let Err(Error::Compiler(branch,token)) = ctx.compile(tokens) {
        branches.push(branch);
        match token.to_lowercase().as_str() {
            "then" => break,
            "else" => continue,
            _ => return Err(Error::Compiler(Cell::Branch(runtime_if, branches), token)),
        }
    }

    Ok(Cell::Branch(runtime_if, branches))
}

pub fn runtime_if(_ctx: &mut Context, _tokens: &mut VecDeque<String>) -> Result<()> {
    //let condition = ctx.value_stack.pop()?;
    //if let Cell::Branch(_,branch) = cell {
    //    if condition == Variable::from(0) && let Some(function) = branch.get(0) {
    //        ctx.execute(function.clone(), tokens)?
    //    }
    //    else if let Some(function) = branch.get(1) {
    //        ctx.execute(function.clone(), tokens)?
    //    }
    //}
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
/// ctx.dictionary.add(".\"",Cell::Compiled(builtins::compiletime_dot_q));
/// ctx.dictionary.add(".", Cell::Exec(builtins::dot));
/// ctx.dictionary.add("+", Cell::Exec(builtins::plus));
///
/// ctx.eval(": foo .\" bar baz \" 1 1 + ; ");
/// ctx.eval(" foo ");
/// assert_eq!(test_stdout.recv().unwrap(), "bar baz");
/// ctx.eval(" . ");
/// assert_eq!(test_stdout.recv().unwrap(), "2");
/// ```
pub fn compiletime_dot_q(_ctx: &mut Context, tokens: &mut VecDeque<String>) -> Result<Cell> {
    let mut buffer = String::new();
    while let Some(token) = tokens.pop_front() {
        if token.ends_with("\"") {
            let comment = Cell::Data(Variable::from(buffer.as_str()));
            let entry = Cell::Exec(runtime_dot_q);
            return Ok(Cell::Routine(vec![comment, entry]));
        }
        if buffer.len() > 0 {
            buffer.push_str(" ");
        }
        buffer.push_str(&token);
    }
    return Err(Error::Parser("EOL".to_owned()));
}

pub fn runtime_dot_q(ctx: &mut Context, _: &mut VecDeque<String>) -> Result<()> {
    let comment = ctx.value_stack.pop()?;
    (ctx.write)(&format!("{}", comment));
    Ok(())
}


/// forth `DO"` command compiletime evaluation
///
/// https://forth-standard.org/standard/core/DO
///
///
/// ```
/// # use frust::*;
/// # use std::sync::mpsc::channel;
/// # let (test_writer, test_stdout) = channel();
/// let mut ctx = Context::new_null();
/// # ctx.write = Box::new( move |str: &str|  {test_writer.send(str.to_owned());});
///
/// ctx.dictionary.add(".\"",Cell::Compiled(builtins::compiletime_dot_q));
/// ctx.dictionary.add(".", Cell::Exec(builtins::dot));
/// ctx.dictionary.add("+", Cell::Exec(builtins::plus));
///
/// ```
pub fn compiletime_do(ctx: &mut Context, tokens: &mut VecDeque<String>) -> Result<Cell> {
    let mut branches = Vec::new();
    let compiled =  ctx.compile(tokens);
    if let Err(Error::Compiler(branch,token)) = compiled {
        branches.push(branch);
        match token.to_lowercase().as_str() {
            "loop" => return Ok(Cell::Branch(runtime_loop, branches)),
            "+loop" => return Ok(Cell::Branch(runtime_plus_loop, branches)),
            "-loop" => return Ok(Cell::Branch(runtime_minus_loop, branches)),
            _ => return Err(Error::Compiler(Cell::Branch(runtime_if, branches), token)),
        }
    }
    else {
        return Err(Error::Compiler(compiled?, "EOL".to_owned()))
    };

}

pub fn runtime_do(_ctx: &mut Context, _: &mut VecDeque<String>) -> Result<()> {
    println!("runtime_do");
    Ok(())
}

pub fn runtime_loop(_ctx: &mut Context, _: &mut VecDeque<String>) -> Result<()> {
    println!("runtime_loop");
    Ok(())
}

pub fn runtime_plus_loop(_ctx: &mut Context, _: &mut VecDeque<String>) -> Result<()> {
    println!("runtime_plus_loop");
    Ok(())
}

pub fn runtime_minus_loop(_ctx: &mut Context, _: &mut VecDeque<String>) -> Result<()> {
    println!("runtime_minus_loop");
    Ok(())
}


