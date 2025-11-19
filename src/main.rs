use frust::*;

fn main() {
    let mut ctx = Context::new_stdio();
    ctx.dictionary.add("+", Cell::Exec(builtins::plus));
    ctx.dictionary.add("-", Cell::Exec(builtins::minus));
    ctx.dictionary.add("*", Cell::Exec(builtins::times));
    ctx.dictionary.add("/", Cell::Exec(builtins::div));
    ctx.dictionary.add("mod", Cell::Exec(builtins::modulo));
    ctx.dictionary.add("\\", Cell::Exec(builtins::lcomment));
    ctx.dictionary.add("(", Cell::Exec(builtins::icomment));
    ctx.dictionary.add(".", Cell::Exec(builtins::dot));
    ctx.dictionary.add("cr", Cell::Exec(builtins::cr));
    ctx.dictionary.add("space", Cell::Exec(builtins::space));
    ctx.dictionary.add("1-", Cell::Exec(builtins::one_minus));
    ctx.dictionary.add("dup", Cell::Exec(builtins::dup));
    ctx.dictionary.add(".s", Cell::Exec(builtins::dot_s));
    ctx.dictionary.add("abs", Cell::Exec(builtins::abs));
    ctx.dictionary.add("=", Cell::Exec(builtins::eq));
    ctx.dictionary.add("max", Cell::Exec(builtins::max));
    ctx.dictionary.add("min", Cell::Exec(builtins::min));
    ctx.dictionary.add("nip", Cell::Exec(builtins::nip));
    ctx.dictionary
        .add("roll", Cell::Exec(builtins::unimplemented));
    ctx.dictionary
        .add("pick", Cell::Exec(builtins::unimplemented));
    ctx.dictionary.add("over", Cell::Exec(builtins::over));
    ctx.dictionary.add("tuck", Cell::Exec(builtins::tuck));
    ctx.dictionary.add("negate", Cell::Exec(builtins::negate));
    ctx.dictionary.add("dup", Cell::Exec(builtins::dup));
    ctx.dictionary.add("swap", Cell::Exec(builtins::swap));
    ctx.dictionary.add("rot", Cell::Exec(builtins::rot));
    ctx.dictionary.add("drop", Cell::Exec(builtins::drop));
    ctx.dictionary.add("?dup", Cell::Exec(builtins::qdup));
    ctx.dictionary.add("i", Cell::Exec(builtins::i));
    ctx.dictionary.add("j", Cell::Exec(builtins::j));
    ctx.dictionary
        .add("if", Cell::Compiled(builtins::compiletime_if));
    ctx.dictionary
        .add("do", Cell::Compiled(builtins::compiletime_do));
    ctx.dictionary
        .add(".\"", Cell::Compiled(builtins::compiletime_dot_q));

    loop {
        let mut buffer = String::new();
        let _ = match (ctx.read)(&mut buffer) {
            Ok(0) => break,
            Ok(_) => ctx.eval(&buffer),
            Err(_) => break,
        };
    }
    println!("bye!")
}
