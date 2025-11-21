use crate::Result;
use crate::VM;

pub fn dot_s(vm: &mut VM) -> Result<()> {
    (vm.write)(&format!(" <{}> ", vm.value_stack.len()));
    for value in vm.value_stack.iter() {
        (vm.write)(&format!(" {} ", value));
    }
    (vm.write)(&format!("\n"));
    Ok(())
}
