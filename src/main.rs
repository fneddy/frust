use frust::{*};

fn main() {
    let mut ctx = Context::new_stdio();
    ctx.dictionary.add("+", Code::Call(builtins::plus));
    ctx.dictionary.add("-", Code::Call(builtins::minus));
    ctx.dictionary.add("*", Code::Call(builtins::times));
    ctx.dictionary.add("/", Code::Call(builtins::div));
    ctx.dictionary
        .add("mod", Code::Call(builtins::modulo));
    ctx.dictionary
        .add("\\", Code::Call(builtins::lcomment));
    ctx.dictionary
        .add("(", Code::Call(builtins::icomment));
    ctx.dictionary.add(".", Code::Call(builtins::dot));
    ctx.dictionary.add("dup", Code::Call(builtins::dup));
    ctx.dictionary.add(".s", Code::Call(builtins::dot_s));
    ctx.dictionary.add("abs", Code::Call(builtins::abs));
    ctx.dictionary.add("=", Code::Call(builtins::eq));
    ctx.dictionary.add("max", Code::Call(builtins::max));
    ctx.dictionary.add("min", Code::Call(builtins::min));
    ctx.dictionary.add("nip", Code::Call(builtins::nip));
    ctx.dictionary
        .add("roll", Code::Call(builtins::unimplemented));
    ctx.dictionary
        .add("pick", Code::Call(builtins::unimplemented));
    ctx.dictionary
        .add("over", Code::Call(builtins::over));
    ctx.dictionary
        .add("tuck", Code::Call(builtins::tuck));
    ctx.dictionary
        .add("negate", Code::Call(builtins::negate));
    ctx.dictionary.add("dup", Code::Call(builtins::dup));
    ctx.dictionary
        .add("swap", Code::Call(builtins::swap));
    ctx.dictionary.add("rot", Code::Call(builtins::rot));
    ctx.dictionary
        .add("drop", Code::Call(builtins::drop));
    ctx.dictionary
        .add("?dup", Code::Call(builtins::qdup));

    ctx.dictionary.add("if",Code::Compiled(builtins::runtime_if, builtins::compiletime_if));
    ctx.dictionary.add("else",Code::Label("ELSE".to_owned()));
    ctx.dictionary.add("then",Code::Label("THEN".to_owned()));
    
    
    ctx.dictionary.add(".\"",Code::Compiled(builtins::runtime_dot_q, builtins::compiletime_dot_q));

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
