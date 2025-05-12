use frust::*;

fn main() {
    let mut ctx = Context::new_stdio();
    ctx.dictionary
        .add(":", Code::Native(builtins::compile));
    ctx.dictionary.add("+", Code::Native(builtins::plus));
    ctx.dictionary.add("-", Code::Native(builtins::minus));
    ctx.dictionary.add("*", Code::Native(builtins::times));
    ctx.dictionary.add("/", Code::Native(builtins::div));
    ctx.dictionary
        .add("mod", Code::Native(builtins::modulo));
    ctx.dictionary
        .add("\\", Code::Native(builtins::lcomment));
    ctx.dictionary
        .add("(", Code::Native(builtins::icomment));
    ctx.dictionary.add(".", Code::Native(builtins::dot));
    ctx.dictionary.add("dup", Code::Native(builtins::dup));
    ctx.dictionary.add(".s", Code::Native(builtins::dot_s));
    ctx.dictionary.add("abs", Code::Native(builtins::abs));
    ctx.dictionary.add("=", Code::Native(builtins::eq));
    ctx.dictionary.add("max", Code::Native(builtins::max));
    ctx.dictionary.add("min", Code::Native(builtins::min));
    ctx.dictionary.add("nip", Code::Native(builtins::nip));
    ctx.dictionary
        .add("roll", Code::Native(builtins::unimplemented));
    ctx.dictionary
        .add("pick", Code::Native(builtins::unimplemented));
    ctx.dictionary
        .add("over", Code::Native(builtins::over));
    ctx.dictionary
        .add("tuck", Code::Native(builtins::tuck));
    ctx.dictionary
        .add("negate", Code::Native(builtins::negate));
    ctx.dictionary.add("dup", Code::Native(builtins::dup));
    ctx.dictionary
        .add("swap", Code::Native(builtins::swap));
    ctx.dictionary.add("rot", Code::Native(builtins::rot));
    ctx.dictionary
        .add("drop", Code::Native(builtins::drop));
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
