use frust::*;

fn main() {
    let mut ctx = Context::new_stdio();
    ctx.dictionary
        .add_code(":", Code::Native(builtins::compile));
    ctx.dictionary.add_code("+", Code::Native(builtins::plus));
    ctx.dictionary.add_code("-", Code::Native(builtins::minus));
    ctx.dictionary.add_code("*", Code::Native(builtins::times));
    ctx.dictionary.add_code("/", Code::Native(builtins::div));
    ctx.dictionary
        .add_code("mod", Code::Native(builtins::modulo));
    ctx.dictionary
        .add_code("\\", Code::Native(builtins::lcomment));
    ctx.dictionary
        .add_code("(", Code::Native(builtins::icomment));
    ctx.dictionary.add_code(".", Code::Native(builtins::dot));
    ctx.dictionary.add_code("dup", Code::Native(builtins::dup));
    ctx.dictionary.add_code(".s", Code::Native(builtins::dot_s));
    ctx.dictionary.add_code("abs", Code::Native(builtins::abs));
    ctx.dictionary.add_code("=", Code::Native(builtins::eq));
    ctx.dictionary.add_code("max", Code::Native(builtins::max));
    ctx.dictionary.add_code("min", Code::Native(builtins::min));
    ctx.dictionary.add_code("nip", Code::Native(builtins::nip));
    ctx.dictionary
        .add_code("roll", Code::Native(builtins::unimplemented));
    ctx.dictionary
        .add_code("pick", Code::Native(builtins::unimplemented));
    ctx.dictionary
        .add_code("over", Code::Native(builtins::over));
    ctx.dictionary
        .add_code("tuck", Code::Native(builtins::tuck));
    ctx.dictionary
        .add_code("negate", Code::Native(builtins::negate));
    ctx.dictionary.add_code("dup", Code::Native(builtins::dup));
    ctx.dictionary
        .add_code("swap", Code::Native(builtins::swap));
    ctx.dictionary.add_code("rot", Code::Native(builtins::rot));
    ctx.dictionary
        .add_code("drop", Code::Native(builtins::drop));
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
