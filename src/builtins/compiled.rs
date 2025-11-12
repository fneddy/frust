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

pub fn compiletime_dot_q(ctx: &mut Context, tokens: &mut VecDeque<String>) -> Result<Cell> {
    let mut buffer = String::new();
    while let Some(token) = tokens.pop_front() {
        
        if token == "\"" {
            let comment = Cell::Data(Variable::from(buffer.as_str()));
            let entry = Cell::Call(runtime_dot_q);
            return Ok(Cell::Routine(vec![comment, entry]));
        }
        buffer.push_str(" ");
        buffer.push_str(&token);
    }
    return Err(Error::Parser);
}
