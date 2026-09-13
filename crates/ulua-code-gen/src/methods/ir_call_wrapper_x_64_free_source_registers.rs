use crate::records::{call_argument::CallArgument, ir_call_wrapper_x_64::IrCallWrapperX64};

impl IrCallWrapperX64 {
  pub fn free_source_registers(&mut self, arg: &mut CallArgument) {
    self.remove_register_use(arg.source.base);
    self.remove_register_use(arg.source.index);
  }
}
