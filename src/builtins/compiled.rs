use std::collections::VecDeque;

use crate::{Context, Error, Result, Variable};

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
pub fn compiletime_if(ctx: &mut Context, _: &mut VecDeque<String>) -> Result<()> {
    todo!()
}


pub fn runtime_dot_q(ctx: &mut Context, _: &mut VecDeque<String>) -> Result<()> {
   todo!()
}

pub fn compiletime_dot_q(ctx: &mut Context, tokens: &mut VecDeque<String>) -> Result<()> {
    let mut buffer = String::new();
    while let Some(token) = tokens.pop_front() {
        
        if token == "\"" {
            return Ok(())
        }
        buffer.push_str(" ");
        buffer.push_str(&token);
    }
    return Err(Error::Parser);
}
