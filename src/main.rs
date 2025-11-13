use frust::*;

fn main() {
    let mut ctx = Context::new_stdio();
    ctx.dictionary.add("+", Cell::Call(builtins::plus));
    ctx.dictionary.add("-", Cell::Call(builtins::minus));
    ctx.dictionary.add("*", Cell::Call(builtins::times));
    ctx.dictionary.add("/", Cell::Call(builtins::div));
    ctx.dictionary.add("mod", Cell::Call(builtins::modulo));
    ctx.dictionary.add("\\", Cell::Call(builtins::lcomment));
    ctx.dictionary.add("(", Cell::Call(builtins::icomment));
    ctx.dictionary.add(".", Cell::Call(builtins::dot));
    ctx.dictionary.add("dup", Cell::Call(builtins::dup));
    ctx.dictionary.add(".s", Cell::Call(builtins::dot_s));
    ctx.dictionary.add("abs", Cell::Call(builtins::abs));
    ctx.dictionary.add("=", Cell::Call(builtins::eq));
    ctx.dictionary.add("max", Cell::Call(builtins::max));
    ctx.dictionary.add("min", Cell::Call(builtins::min));
    ctx.dictionary.add("nip", Cell::Call(builtins::nip));
    ctx.dictionary
        .add("roll", Cell::Call(builtins::unimplemented));
    ctx.dictionary
        .add("pick", Cell::Call(builtins::unimplemented));
    ctx.dictionary.add("over", Cell::Call(builtins::over));
    ctx.dictionary.add("tuck", Cell::Call(builtins::tuck));
    ctx.dictionary.add("negate", Cell::Call(builtins::negate));
    ctx.dictionary.add("dup", Cell::Call(builtins::dup));
    ctx.dictionary.add("swap", Cell::Call(builtins::swap));
    ctx.dictionary.add("rot", Cell::Call(builtins::rot));
    ctx.dictionary.add("drop", Cell::Call(builtins::drop));
    ctx.dictionary.add("?dup", Cell::Call(builtins::qdup));

    ctx.dictionary.add(
        "if",
        Cell::Compiled(builtins::runtime_if, builtins::compiletime_if),
    );
    ctx.dictionary.add("else", Cell::Label("ELSE".to_owned()));
    ctx.dictionary.add("then", Cell::Label("THEN".to_owned()));

    ctx.dictionary.add(
        ".\"",
        Cell::Compiled(builtins::runtime_dot_q, builtins::compiletime_dot_q),
    );

    loop {
        let mut buffer = String::new();
        match (ctx.read)(&mut buffer) {
            Ok(0) => break,
            Ok(_) => match ctx.eval(&buffer) {
                Ok(()) => (ctx.write)("ok"),
                Err(v) => (ctx.write)(&format!("??? {:?}", v)),
            },
            Err(_) => break,
        };
    }
    println!("bye!")
}
