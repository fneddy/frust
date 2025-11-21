use crate::*;

/// forth `if` command compiletime evaluation
///
/// https://forth-standard.org/standard/core/IF
///
/// ```
/// # use frust::*;
/// # use std::time::Duration;
/// # use std::sync::mpsc::channel;
/// # let (test_writer, test_stdout) = channel();
/// let mut vm = VM::new_null();
/// # vm.write = Box::new( move |str: &str|  {test_writer.send(str.to_owned());});
///
/// vm.dictionary.add("if",Cell::Compiler(builtins::compiletime_if));
/// vm.dictionary.add(".\"",Cell::Compiler(builtins::compiletime_dot_q));
/// vm.dictionary.add(".", Cell::Exec(builtins::dot));
///
/// vm.eval(": foo IF . ELSE .\" No more \" THEN ; ");
/// vm.eval(" 1 1 foo ");
/// assert_eq!(test_stdout.recv_timeout(Duration::from_millis(400)).unwrap(), "1");
/// vm.eval(" 0 foo ");
/// assert_eq!(test_stdout.recv_timeout(Duration::from_millis(400)).unwrap(), "No more");

/// ```
pub fn compiletime_if(vm: &mut VM) -> Result<Vec<Cell>> {
    let mut branch_true: Vec<Cell> = Vec::new();
    let mut branch_false: Vec<Cell> = Vec::new();
    let mut next_token = String::new();

    if let Err(Error::Compiler(branch, token)) = vm.compile() {
        branch_true = branch;
        next_token = token.to_lowercase();
    }

    if next_token == "else" {
        if let Err(Error::Compiler(branch, token)) = vm.compile() {
            branch_false = branch;
            next_token = token.to_lowercase();
        }
    }

    if next_token == "then" {
        let program: Vec<Cell> = vec![
            Cell::Data(Variable::Int(branch_true.len() as i64 + 3)),
            Cell::ControlBranchIfZero,
        ]
        .into_iter()
        .chain(branch_true.into_iter())
        .chain(
            vec![
                Cell::Data(Variable::Int(branch_false.len() as i64 + 1)),
                Cell::ControlBranch,
            ]
            .into_iter(),
        )
        .chain(branch_false.into_iter())
        .collect();
        return Ok(program);
    }
    Err(Error::Compiler(vec![], next_token))
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
/// # use std::time::Duration;
/// # use std::sync::mpsc::channel;
/// # let (test_writer, test_stdout) = channel();
/// let mut vm = VM::new_null();
/// # vm.write = Box::new( move |str: &str|  {test_writer.send(str.to_owned());});
///
/// vm.dictionary.add(".\"",Cell::Compiler(builtins::compiletime_dot_q));
/// vm.dictionary.add(".", Cell::Exec(builtins::dot));
/// vm.dictionary.add("+", Cell::Exec(builtins::plus));
///
/// vm.eval(": foo .\" bar baz \" 1 1 + ; ");
/// vm.eval(" foo ");
/// assert_eq!(test_stdout.recv_timeout(Duration::from_millis(400)).unwrap(), "bar baz");
/// vm.eval(" . ");
/// assert_eq!(test_stdout.recv_timeout(Duration::from_millis(400)).unwrap(), "2");
/// ```
pub fn compiletime_dot_q(vm: &mut VM) -> Result<Vec<Cell>> {
    let mut buffer = String::new();
    while let Some(token) = vm.input_buffer.pop_front() {
        if token.ends_with("\"") && token.len() > 1 {
            buffer.push_str(token.trim_end_matches('"'));
        }

        if token.ends_with("\"") {
            let comment = Cell::Data(Variable::from(buffer.as_str()));
            let entry = Cell::Exec(runtime_dot_q);
            return Ok(vec![comment, entry]);
        }
        if buffer.len() > 0 {
            buffer.push_str(" ");
        }
        buffer.push_str(&token);
    }
    return Err(Error::Parser("EOL".to_owned()));
}

pub fn runtime_dot_q(vm: &mut VM) -> Result<()> {
    let comment = vm.value_stack.pop()?;
    (vm.write)(&format!("{}", comment));
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
/// let mut vm = VM::new_null();
/// # vm.write = Box::new( move |str: &str|  {test_writer.send(str.to_owned());});
///
/// vm.dictionary.add(".\"",Cell::Compiler(builtins::compiletime_dot_q));
/// vm.dictionary.add(".", Cell::Exec(builtins::dot));
/// vm.dictionary.add("+", Cell::Exec(builtins::plus));
///
/// ```
pub fn compiletime_do(vm: &mut VM) -> Result<Vec<Cell>> {
    let mut branch = vec![Cell::Exec(runtime_do)];

    let compiled = vm.compile();
    if let Err(Error::Compiler(routine, token)) = compiled {
        match token.to_lowercase().as_str() {
            "loop" => {
                let len = routine.len() as i64 + 2;
                branch.extend(routine);
                branch.push(Cell::Exec(runtime_loop));
                branch.push(Cell::Data(Variable::Int(len * -1)));
                branch.push(Cell::ControlBranchIfNotZero);
                return Ok(branch);
            }
            "+loop" => {
                let len = routine.len() as i64 + 2;
                branch.extend(routine);
                branch.push(Cell::Exec(runtime_plus_loop));
                branch.push(Cell::Data(Variable::Int(len * -1)));
                branch.push(Cell::ControlBranchIfNotZero);
                return Ok(branch);
            }
            "-loop" => {
                let len = routine.len() as i64 + 2;
                branch.extend(routine);
                branch.push(Cell::Exec(runtime_minus_loop));
                branch.push(Cell::Data(Variable::Int(len * -1)));
                branch.push(Cell::ControlBranchIfNotZero);
                return Ok(branch);
            }
            _ => return Err(Error::Compiler(branch, token)),
        }
    } else {
        return Err(Error::Compiler(compiled?, "EOL".to_owned()));
    };
}

pub fn runtime_do(vm: &mut VM) -> Result<()> {
    let index = vm.value_stack.pop()?;
    let limit = vm.value_stack.pop()?;
    vm.return_stack.push(limit);
    vm.return_stack.push(index);
    Ok(())
}

pub fn runtime_loop(vm: &mut VM) -> Result<()> {
    let i_next = vm.return_stack.pop()? + variable::Variable::Int(1);
    let limit = vm.return_stack.pop()?;

    if i_next < limit {
        vm.return_stack.push(limit);
        vm.return_stack.push(i_next.clone());
        vm.value_stack.push(1);
    } else {
        vm.value_stack.push(0);
    }
    Ok(())
}

pub fn runtime_plus_loop(vm: &mut VM) -> Result<()> {
    let offset = vm.value_stack.pop()?;
    let i_next = vm.return_stack.pop()? + offset;
    let limit = vm.return_stack.pop()?;

    if i_next < limit {
        vm.return_stack.push(limit);
        vm.return_stack.push(i_next.clone());
        vm.value_stack.push(1);
    } else {
        vm.value_stack.push(0);
    }
    Ok(())
}

pub fn runtime_minus_loop(vm: &mut VM) -> Result<()> {
    let offset = vm.value_stack.pop()?;
    let i_next = vm.return_stack.pop()? - offset;
    let limit = vm.return_stack.pop()?;

    if i_next > limit {
        vm.return_stack.push(limit);
        vm.return_stack.push(i_next.clone());
        vm.value_stack.push(1);
    } else {
        vm.value_stack.push(0);
    }
    Ok(())
}
