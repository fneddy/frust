use std::collections::VecDeque;

use crate::{Error, Result, Variable};

use crate::Context;

/// not a real forth command
/// 
/// - dummy that always fails
/// 
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::unimplemented;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// assert_eq!(unimplemented(&mut ctx, &mut empty), Err(Error::Unimplemented));
/// 
/// ```
pub fn unimplemented(_ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    Err(Error::Unimplemented)
}

/// forth `dot` command `.`
///
/// https://forth-standard.org/standard/core/d
///
/// - pops last element from the value stack
/// - prints to the user
///
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::dot;
/// # let mut empty = VecDeque::new();
/// # let mut ctx = Context::new_null();
/// ctx.value_stack.push(23);
/// let _ = dot(&mut ctx, &mut empty);
/// let _ = dot(&mut ctx, &mut empty);
///
/// // expected output:
/// // 23
/// // Stack error
///
/// ```
pub fn dot(ctx: &mut Context, _: &mut VecDeque<&str>) -> Result<()> {
    let v = ctx.value_stack.pop()?;
    (ctx.write)(&format!("{}", v));
    Ok(())
}

/// forth `+` command
///
/// https://forth-standard.org/standard/core/Plus
///
/// - pops two elements from the value stack,
/// - adds them
/// - pushes result back to value stack
///
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::plus;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(23);
/// ctx.value_stack.push(42);
///
/// plus(&mut ctx, &mut empty);
///
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(65)));
///
/// ```
pub fn plus(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    let b = ctx.value_stack.pop()?;
    let a = ctx.value_stack.pop()?;
    ctx.value_stack.push(a + b);
    Ok(())
}

/// forth `-` command
///
/// https://forth-standard.org/standard/core/Minus
///
/// - pops two elements from the value stack,
/// - subtracts first from second
/// - pushes result back to value stack
///
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::minus;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(23);
/// ctx.value_stack.push(42);
///
/// minus(&mut ctx, &mut empty);
///
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(-19)))
///
/// ```
pub fn minus(ctx: &mut Context, _: &mut VecDeque<&str>) -> Result<()> {
    let b = ctx.value_stack.pop()?;
    let a = ctx.value_stack.pop()?;
    ctx.value_stack.push(a - b);
    Ok(())
}

/// /// forth `*` command
///
/// https://forth-standard.org/standard/core/Times
///
/// - pops two elements from the value stack,
/// - multiplies them
/// - pushes result back to value stack
///
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::times;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(4);
/// ctx.value_stack.push(3);
///
/// times(&mut ctx, &mut empty);
///
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(12)))
///
/// ```
pub fn times(ctx: &mut Context, _: &mut VecDeque<&str>) -> Result<()> {
    let b = ctx.value_stack.pop()?;
    let a = ctx.value_stack.pop()?;
    ctx.value_stack.push(a * b);
    Ok(())
}

/// forth `max` command
///
/// https://forth-standard.org/standard/core/MAX
///
/// - pops two elements from the value stack,
/// - compares them
/// - pushes grater value back to value stack
///
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::max;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(4);
/// ctx.value_stack.push(3);
///
/// max(&mut ctx, &mut empty);
///
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(4)))
///
/// ```
pub fn max(ctx: &mut Context, _: &mut VecDeque<&str>) -> Result<()> {
    let b = ctx.value_stack.pop()?;
    let a = ctx.value_stack.pop()?;
    if a > b {
        ctx.value_stack.push(a);
    } else {
        ctx.value_stack.push(b);
    }
    Ok(())
}

/// forth `min` command
///
/// https://forth-standard.org/standard/core/MIN
///
/// - pops two elements from the value stack,
/// - compares them
/// - pushes smaller value back to value stack
///
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::min;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(4);
/// ctx.value_stack.push(3);
///
/// min(&mut ctx, &mut empty);
///
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(3)))
///
/// ```
pub fn min(ctx: &mut Context, _: &mut VecDeque<&str>) -> Result<()> {
    let b = ctx.value_stack.pop()?;
    let a = ctx.value_stack.pop()?;
    if a < b {
        ctx.value_stack.push(a);
    } else {
        ctx.value_stack.push(b);
    }
    Ok(())
}

/// forth `/` command
///
/// https://forth-standard.org/standard/core/Div
///
/// - pops two elements from the value stack,
/// - divides fist by second,
/// - pushes result back to value stack,
///
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::div;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(8);
/// ctx.value_stack.push(2);
///
/// div(&mut ctx, &mut empty);
///
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(4)))
///
/// ```
pub fn div(ctx: &mut Context, _: &mut VecDeque<&str>) -> Result<()> {
    let b = ctx.value_stack.pop()?;
    let a = ctx.value_stack.pop()?;
    ctx.value_stack.push(a / b);
    Ok(())
}

/// forth `/mod` command
///
/// https://forth-standard.org/standard/core/DivMOD
///
/// - pops two elements from the value stack,
/// - divides fist by second,
/// - pushes result back to value stack,
/// - pushes back reminder to value stack,
///
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::modulo;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(9);
/// ctx.value_stack.push(2);
///
/// modulo(&mut ctx, &mut empty);
///
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(4)));
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(1)));
///
/// ```
pub fn modulo(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    let b = ctx.value_stack.pop()?;
    let a = ctx.value_stack.pop()?;
    ctx.value_stack.push(a.clone() % b.clone());
    ctx.value_stack.push(a / b);
    Ok(())
}

/// forth `dup` command
/// 
/// https://forth-standard.org/standard/core/DUP
/// 
/// - duplicates the last element of the value stack
/// 
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::dup;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(9);
///
/// dup(&mut ctx, &mut empty);
/// 
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(9)));
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(9)));
///
/// ```
/// 
pub fn dup(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    let value = ctx.value_stack.pop()?;
    ctx.value_stack.push(value.clone());
    ctx.value_stack.push(value);
    Ok(())
}

/// forth `swap` command
///
/// https://forth-standard.org/standard/core/SWAP
/// 
/// - exchange the last two elements on value stack
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::swap;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(9);
/// ctx.value_stack.push(1);
///
/// swap(&mut ctx, &mut empty);
/// 
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(9)));
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(1)));
/// 
/// ```
pub fn swap(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    let a = ctx.value_stack.pop()?;
    let b = ctx.value_stack.pop()?;
    ctx.value_stack.push(a);
    ctx.value_stack.push(b);
    Ok(())
}

/// forth `rot` command
/// 
/// https://forth-standard.org/standard/core/ROT
/// 
/// - rotate last 3 value on value stack
/// 
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::rot;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(1);
/// ctx.value_stack.push(2);
/// ctx.value_stack.push(3);
///
/// rot(&mut ctx, &mut empty);
/// 
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(1)));
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(3)));
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(2)));
/// ```
pub fn rot(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    let x3 = ctx.value_stack.pop()?;
    let x2 = ctx.value_stack.pop()?;
    let x1 = ctx.value_stack.pop()?;
    ctx.value_stack.push(x2);
    ctx.value_stack.push(x3);
    ctx.value_stack.push(x1);
    Ok(())
}

/// the forth `nip` command
/// 
/// https://forth-standard.org/standard/core/NIP
/// 
/// - pop two values
/// - push back only top value
/// 
/// electively deletes the value for the top value
/// 
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::nip;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(1);
/// ctx.value_stack.push(2);
/// ctx.value_stack.push(3);
///
/// nip(&mut ctx, &mut empty);
/// 
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(3)));
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(1)));
/// ```
pub fn nip(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    let value = ctx.value_stack.pop()?;
    let _ = ctx.value_stack.pop()?;
    ctx.value_stack.push(value);
    Ok(())
}

/// the forth `tuck` command
/// 
/// - pop two elements
/// - push back in the same order
/// - push back copy of last element
/// 
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::tuck;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(1);
/// ctx.value_stack.push(2);
///
/// tuck(&mut ctx, &mut empty);
/// 
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(2)));
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(1)));
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(2)));
/// ```
/// 
pub fn tuck(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    let b = ctx.value_stack.pop()?;
    let a = ctx.value_stack.pop()?;
    ctx.value_stack.push(b.clone());
    ctx.value_stack.push(a);
    ctx.value_stack.push(b);
    Ok(())
}

/// the forth `over` command
/// 
/// https://forth-standard.org/standard/core/OVER
/// 
/// - pop two elements
/// - push back copy of last element
/// - push back in the same order
/// 
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::over;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(1);
/// ctx.value_stack.push(2);
///
/// over(&mut ctx, &mut empty);
/// 
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(1)));
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(2)));
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(1)));
/// ```
pub fn over(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    let b = ctx.value_stack.pop()?;
    let a = ctx.value_stack.pop()?;
    ctx.value_stack.push(a.clone());
    ctx.value_stack.push(b);
    ctx.value_stack.push(a);
    Ok(())
}

/// forth `drop` command
/// 
/// removes last element from stack
/// 
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::drop;
/// # let mut empty = VecDeque::new();
/// let mut ctx = Context::new_null();
/// ctx.value_stack.push(1);
/// ctx.value_stack.push(2);
///
/// drop(&mut ctx, &mut empty);
/// 
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(1)));
/// ```
/// 
pub fn drop(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    let _ = ctx.value_stack.pop()?;
    Ok(())
}

/// forth line comment
/// - drops everything till end of line
/// as we only process buffers line by line
/// its save to just clear the complete buffer here
pub fn lcomment(_ctx: &mut Context, buffer: &mut VecDeque<&str>) -> Result<()> {
    buffer.clear();
    Ok(())
}

/// forth `()` comment
/// - drops everything till closing `)`
/// 
/// https://forth-standard.org/standard/core/p
/// 
/// ```
/// # use std::collections::VecDeque;
/// # use frust::*;
/// # use frust::builtins::icomment;
/// let mut buffer: VecDeque<&str>  = vec!["this", "is", "a", "tokenized", "comment)", "forth_code"].into();
/// let mut ctx = Context::new_null();
///
/// icomment(&mut ctx, &mut buffer);
/// 
/// assert_eq!(buffer.pop_front(), Some("forth_code"));
/// 
/// ```
pub fn icomment(_ctx: &mut Context, buffer: &mut VecDeque<&str>) -> Result<()> {
    while let Some(v) = buffer.pop_front() {
        if v.ends_with(")") {
            return Ok(());
        }
    }
    Err(Error::Parser)
}

/// forth `negate` command
/// 
/// https://forth-standard.org/standard/core/NEGATE
/// 
/// - negates last stack value if its integer
/// 
/// ```
/// # use std::collections::VecDeque;
/// # use frust::*;
/// # use frust::builtins::negate;
/// # let mut empty: VecDeque<&str>  = vec![].into();
/// let mut ctx = Context::new_null();
///
/// ctx.value_stack.push(Variable::Int(1));
/// 
/// negate(&mut ctx, &mut empty);
/// 
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(-1)));
/// 
/// ```
/// 
pub fn negate(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    if let Ok(v) = ctx.value_stack.at_mut(0) {
        match v {
            Variable::Int(v) => *v = *v * -1,
            _ => {}
        }
    }
    Ok(())
}

/// forth `=` command
/// 
/// https://forth-standard.org/standard/core/Equal
/// 
/// - pops last two elements from stack
/// - compares them
/// - if equal writes `true` (-1) on the stack
/// - else 0 
/// 
/// ```
/// # use std::collections::VecDeque;
/// # use frust::*;
/// # use frust::builtins::eq;
/// # let mut empty: VecDeque<&str>  = vec![].into();
/// let mut ctx = Context::new_null();
///
/// ctx.value_stack.push(1);
/// ctx.value_stack.push(1);
/// 
/// eq(&mut ctx, &mut empty);
/// 
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(-1)));
/// 
/// ctx.value_stack.push(Variable::String("foo".into()));
/// ctx.value_stack.push(Variable::String("foo".into()));
/// 
/// eq(&mut ctx, &mut empty);
/// 
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(-1)));
/// 
/// ```
/// 
pub fn eq(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    let a = ctx.value_stack.pop()?;
    let b = ctx.value_stack.pop()?;
    if a == b {
        ctx.value_stack.push(Variable::Int(-1));
    } else {
        ctx.value_stack.push(Variable::Int(0));
    }
    Ok(())
}

/// forth `abs` command
/// 
/// https://forth-standard.org/standard/core/ABS
/// 
/// - pops last element
/// - pushes abs value of last element
/// 
/// ```
/// # use std::collections::VecDeque;
/// # use frust::*;
/// # use frust::builtins::abs;
/// # let mut empty: VecDeque<&str>  = vec![].into();
/// let mut ctx = Context::new_null();
///
/// ctx.value_stack.push(Variable::Int(-1));
/// 
/// abs(&mut ctx, &mut empty);
/// 
/// assert_eq!(ctx.value_stack.pop(), Ok(Variable::Int(1)));
/// 
/// ```
pub fn abs(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    if let Ok(v) = ctx.value_stack.at_mut(0) {
        match v {
            Variable::Int(v) => *v = v.abs(),
            _ => {}
        }
    }
    Ok(())
}

/// forth `cr` command
/// 
/// prints `\n` to write
pub fn cr(ctx: &mut Context, _: &mut VecDeque<&str>) -> Result<()> {
    (ctx.write)(&format!("\n"));
    Ok(())
}


pub fn compile(_ctx: &mut Context, _: &mut VecDeque<&str>) -> Result<()> {
    
    Err(Error::Parser)
}
