use crate::Result;
use crate::VM;

pub fn dot_s(vm: &mut VM) -> Result<()> {
    for value in vm.value_stack.iter() {
        (vm.write)(&format!(" {} ", value));
    }
    (vm.write)(&format!("\n"));
    Ok(())
}
