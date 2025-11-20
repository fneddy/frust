use crate::{Error, Result, Variable};

use crate::VM;

/// not a real forth command
///
/// - dummy that always fails
///
/// ```
/// # use frust::*;
/// # use std::collections::VecDeque;
/// # use frust::builtins::unimplemented;
/// let mut vm = VM::new_null();
/// assert_eq!(unimplemented(&mut vm), Err(Error::Unimplemented("Function".to_owned())));
///
/// ```
pub fn unimplemented(_: &mut VM) -> Result<()> {
    Err(Error::Unimplemented("Function".to_owned()))
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
/// # use std::sync::mpsc::channel;
/// # use std::collections::VecDeque;
/// # use frust::builtins::dot;
/// # let (test_writer, test_stdout) = channel();
/// let mut vm = VM::new_null();
/// # vm.write = Box::new( move |str: &str|  {test_writer.send(str.to_owned());});
/// 
/// vm.value_stack.push(23);
/// 
/// assert_eq!(dot(&mut vm),Ok(()));
/// assert_eq!(test_stdout.recv().unwrap(), "23");
/// 
/// assert_eq!(dot(&mut vm), Err(Error::Stack));
///
/// ```
pub fn dot(vm: &mut VM) -> Result<()> {
    let v = vm.value_stack.pop()?;
    (vm.write)(&format!("{}", v));
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(23);
/// vm.value_stack.push(42);
///
/// plus(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(65)));
///
/// ```
pub fn plus(vm: &mut VM) -> Result<()> {
    let b = vm.value_stack.pop()?;
    let a = vm.value_stack.pop()?;
    vm.value_stack.push(a + b);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(23);
/// vm.value_stack.push(42);
///
/// minus(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(-19)))
///
/// ```
pub fn minus(vm: &mut VM) -> Result<()> {
    let b = vm.value_stack.pop()?;
    let a = vm.value_stack.pop()?;
    vm.value_stack.push(a - b);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(4);
/// vm.value_stack.push(3);
///
/// times(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(12)))
///
/// ```
pub fn times(vm: &mut VM) -> Result<()> {
    let b = vm.value_stack.pop()?;
    let a = vm.value_stack.pop()?;
    vm.value_stack.push(a * b);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(4);
/// vm.value_stack.push(3);
///
/// max(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(4)))
///
/// ```
pub fn max(vm: &mut VM) -> Result<()> {
    let b = vm.value_stack.pop()?;
    let a = vm.value_stack.pop()?;
    if a > b {
        vm.value_stack.push(a);
    } else {
        vm.value_stack.push(b);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(4);
/// vm.value_stack.push(3);
///
/// min(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(3)))
///
/// ```
pub fn min(vm: &mut VM) -> Result<()> {
    let b = vm.value_stack.pop()?;
    let a = vm.value_stack.pop()?;
    if a < b {
        vm.value_stack.push(a);
    } else {
        vm.value_stack.push(b);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(8);
/// vm.value_stack.push(2);
///
/// div(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(4)))
///
/// ```
pub fn div(vm: &mut VM) -> Result<()> {
    let b = vm.value_stack.pop()?;
    let a = vm.value_stack.pop()?;
    vm.value_stack.push(a / b);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(9);
/// vm.value_stack.push(2);
///
/// modulo(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(4)));
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(1)));
///
/// ```
pub fn modulo(vm: &mut VM) -> Result<()> {
    let b = vm.value_stack.pop()?;
    let a = vm.value_stack.pop()?;
    vm.value_stack.push(a.clone() % b.clone());
    vm.value_stack.push(a / b);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(9);
///
/// dup(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(9)));
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(9)));
///
/// ```
///
pub fn dup(vm: &mut VM) -> Result<()> {
    let value = vm.value_stack.pop()?;
    vm.value_stack.push(value.clone());
    vm.value_stack.push(value);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(9);
/// vm.value_stack.push(1);
///
/// swap(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(9)));
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(1)));
///
/// ```
pub fn swap(vm: &mut VM) -> Result<()> {
    let a = vm.value_stack.pop()?;
    let b = vm.value_stack.pop()?;
    vm.value_stack.push(a);
    vm.value_stack.push(b);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(1);
/// vm.value_stack.push(2);
/// vm.value_stack.push(3);
///
/// rot(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(1)));
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(3)));
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(2)));
/// ```
pub fn rot(vm: &mut VM) -> Result<()> {
    let x3 = vm.value_stack.pop()?;
    let x2 = vm.value_stack.pop()?;
    let x1 = vm.value_stack.pop()?;
    vm.value_stack.push(x2);
    vm.value_stack.push(x3);
    vm.value_stack.push(x1);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(1);
/// vm.value_stack.push(2);
/// vm.value_stack.push(3);
///
/// nip(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(3)));
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(1)));
/// ```
pub fn nip(vm: &mut VM) -> Result<()> {
    let value = vm.value_stack.pop()?;
    let _ = vm.value_stack.pop()?;
    vm.value_stack.push(value);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(1);
/// vm.value_stack.push(2);
///
/// tuck(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(2)));
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(1)));
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(2)));
/// ```
///
pub fn tuck(vm: &mut VM) -> Result<()> {
    let b = vm.value_stack.pop()?;
    let a = vm.value_stack.pop()?;
    vm.value_stack.push(b.clone());
    vm.value_stack.push(a);
    vm.value_stack.push(b);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(1);
/// vm.value_stack.push(2);
///
/// over(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(1)));
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(2)));
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(1)));
/// ```
pub fn over(vm: &mut VM) -> Result<()> {
    let b = vm.value_stack.pop()?;
    let a = vm.value_stack.pop()?;
    vm.value_stack.push(a.clone());
    vm.value_stack.push(b);
    vm.value_stack.push(a);
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
/// let mut vm = VM::new_null();
/// vm.value_stack.push(1);
/// vm.value_stack.push(2);
///
/// drop(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(1)));
/// ```
///
pub fn drop(vm: &mut VM) -> Result<()> {
    let _ = vm.value_stack.pop()?;
    Ok(())
}

/// forth line comment
/// - drops everything till end of line
/// as we only process buffers line by line
/// its save to just clear the complete buffer here
pub fn lcomment(vm: &mut VM) -> Result<()> {
    vm.input_buffer.clear();
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
/// let mut vm = VM::new_null();
/// vm.input_buffer = vec!["this".to_owned(), "is".to_owned(), "a".to_owned(), "tokenized".to_owned(), "comment)".to_owned(), "forth_code".to_owned()].into();
///
/// icomment(&mut vm);
///
/// assert_eq!(vm.input_buffer.pop_front(), Some("forth_code".to_owned()));
///
/// ```
pub fn icomment(vm: &mut VM) -> Result<()> {
    while let Some(v) = vm.input_buffer.pop_front() {
        if v.ends_with(")") {
            return Ok(());
        }
    }
    Err(Error::Parser("EOL".to_owned()))
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
/// # let mut empty: VecDeque<String>  = vec![].into();
/// let mut vm = VM::new_null();
///
/// vm.value_stack.push(Variable::Int(1));
///
/// negate(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(-1)));
///
/// ```
///
pub fn negate(vm: &mut VM) -> Result<()> {
    if let Ok(v) = vm.value_stack.at_mut(0) {
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
/// let mut vm = VM::new_null();
///
/// vm.value_stack.push(1);
/// vm.value_stack.push(1);
///
/// eq(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(-1)));
///
/// vm.value_stack.push(Variable::String("foo".into()));
/// vm.value_stack.push(Variable::String("foo".into()));
///
/// eq(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(-1)));
///
/// ```
///
pub fn eq(vm: &mut VM) -> Result<()> {
    let a = vm.value_stack.pop()?;
    let b = vm.value_stack.pop()?;
    if a == b {
        vm.value_stack.push(Variable::Int(-1));
    } else {
        vm.value_stack.push(Variable::Int(0));
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
/// let mut vm = VM::new_null();
///
/// vm.value_stack.push(Variable::Int(-1));
///
/// abs(&mut vm);
///
/// assert_eq!(vm.value_stack.pop(), Ok(Variable::Int(1)));
///
/// ```
pub fn abs(vm: &mut VM) -> Result<()> {
    if let Ok(v) = vm.value_stack.at_mut(0) {
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
pub fn cr(vm: &mut VM) -> Result<()> {
    (vm.write)(&format!("\n"));
    Ok(())
}

/// forth `space` command
///
/// https://forth-standard.org/standard/core/SPACE
/// 
/// prints ` ` to write
pub fn space(vm: &mut VM) -> Result<()> {
    (vm.write)(&format!(" "));
    Ok(())
}

/// forth `1-` command
///
/// https://forth-standard.org/standard/core/OneMinus
/// 
/// prints ` ` to write
pub fn one_minus(vm: &mut VM) -> Result<()> {
     if let Ok(v) = vm.value_stack.at_mut(0) {
        match v {
            Variable::Int(v) => *v = *v - 1,
            _ => {}
        }
    }
    Ok(())
}


/// forth `?dup` command
///
/// https://forth-standard.org/standard/core/qDUP
///
///
pub fn qdup(vm: &mut VM) -> Result<()> {
    if let Ok(v) = vm.value_stack.at(0) {
        if *v != Variable::Int(0) {
            vm.value_stack.push(v.clone());
        }
    }
    Ok(())
}

/// forth `I` command
///
/// https://forth-standard.org/standard/core/I
///
///
pub fn i(vm: &mut VM) -> Result<()> {
    let idx = vm.return_stack.at(0)?;
    vm.value_stack.push(idx);
    Ok(())
}


/// forth `J` command
///
/// https://forth-standard.org/standard/core/J
///
///
pub fn j(vm: &mut VM) -> Result<()> {
    let idx = vm.return_stack.at(1)?;
    vm.value_stack.push(idx);
    Ok(())
}
