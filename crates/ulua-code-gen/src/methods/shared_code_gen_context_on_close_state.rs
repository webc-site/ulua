use crate::records::shared_code_gen_context::SharedCodeGenContext;

impl SharedCodeGenContext {
  pub fn on_close_state(&mut self) {
    // The lifetime of the SharedCodeGenContext is managed separately from the
    // VMs that use it. When a VM is destroyed, we don't need to do anything
    // here.
  }
}

pub fn shared_code_gen_context_on_close_state(this: &mut SharedCodeGenContext) {
  this.on_close_state();
}
