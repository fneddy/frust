use std::collections::VecDeque;

use crate::Result;

use crate::Context;

pub fn dot_s(ctx: &mut Context, _buffer: &mut VecDeque<&str>) -> Result<()> {
    for value in ctx.value_stack.iter() {
        (ctx.write)(&format!("{}", value));
    }
    Ok(())
}
