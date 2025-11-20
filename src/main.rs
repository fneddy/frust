use frust::*;

fn main() {
    let mut vm = VM::new_stdio();
    vm.dictionary.add("+", Cell::Exec(builtins::plus));
    vm.dictionary.add("-", Cell::Exec(builtins::minus));
    vm.dictionary.add("*", Cell::Exec(builtins::times));
    vm.dictionary.add("/", Cell::Exec(builtins::div));
    vm.dictionary.add("mod", Cell::Exec(builtins::modulo));
    vm.dictionary.add("\\", Cell::Exec(builtins::lcomment));
    vm.dictionary.add("(", Cell::Exec(builtins::icomment));
    vm.dictionary.add(".", Cell::Exec(builtins::dot));
    vm.dictionary.add("cr", Cell::Exec(builtins::cr));
    vm.dictionary.add("space", Cell::Exec(builtins::space));
    vm.dictionary.add("1-", Cell::Exec(builtins::one_minus));
    vm.dictionary.add("dup", Cell::Exec(builtins::dup));
    vm.dictionary.add(".s", Cell::Exec(builtins::dot_s));
    vm.dictionary.add("abs", Cell::Exec(builtins::abs));
    vm.dictionary.add("=", Cell::Exec(builtins::eq));
    vm.dictionary.add("max", Cell::Exec(builtins::max));
    vm.dictionary.add("min", Cell::Exec(builtins::min));
    vm.dictionary.add("nip", Cell::Exec(builtins::nip));
    vm.dictionary
        .add("roll", Cell::Exec(builtins::unimplemented));
    vm.dictionary
        .add("pick", Cell::Exec(builtins::unimplemented));
    vm.dictionary.add("over", Cell::Exec(builtins::over));
    vm.dictionary.add("tuck", Cell::Exec(builtins::tuck));
    vm.dictionary.add("negate", Cell::Exec(builtins::negate));
    vm.dictionary.add("dup", Cell::Exec(builtins::dup));
    vm.dictionary.add("swap", Cell::Exec(builtins::swap));
    vm.dictionary.add("rot", Cell::Exec(builtins::rot));
    vm.dictionary.add("drop", Cell::Exec(builtins::drop));
    vm.dictionary.add("?dup", Cell::Exec(builtins::qdup));
    vm.dictionary.add("i", Cell::Exec(builtins::i));
    vm.dictionary.add("j", Cell::Exec(builtins::j));
    vm.dictionary
        .add("if", Cell::Compiler(builtins::compiletime_if));
    vm.dictionary
        .add("do", Cell::Compiler(builtins::compiletime_do));
    vm.dictionary
        .add(".\"", Cell::Compiler(builtins::compiletime_dot_q));

    loop {
        let mut buffer = String::new();
        let _ = match (vm.read)(&mut buffer) {
            Ok(0) => break,
            Ok(_) => vm.eval(&buffer),
            Err(_) => break,
        };
    }
    println!("bye!")
}
