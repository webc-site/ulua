use crate::records::{ir_call_wrapper_x_64::IrCallWrapperX64, register_x_64::RegisterX64};

impl IrCallWrapperX64 {
  pub fn find_conflicting_target(&self) -> RegisterX64 {
    for i in 0..(self.arg_count as usize) {
      let arg = &self.args[i];

      if arg.candidate {
        if self.interferes_with_active_target(arg.source.base) {
          return arg.source.base;
        }

        if self.interferes_with_active_target(arg.source.index) {
          return arg.source.index;
        }
      }
    }

    if self.interferes_with_active_target(self.func_op.base) {
      return self.func_op.base;
    }

    if self.interferes_with_active_target(self.func_op.index) {
      return self.func_op.index;
    }

    RegisterX64::NOREG
  }
}
