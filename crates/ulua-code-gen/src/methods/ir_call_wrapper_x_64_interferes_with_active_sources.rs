use crate::records::{call_argument::CallArgument, ir_call_wrapper_x_64::IrCallWrapperX64};

impl IrCallWrapperX64 {
  #[inline]
  pub fn interferes_with_active_sources(
    &self,
    target_arg: &CallArgument,
    target_arg_index: i32,
  ) -> bool {
    for i in 0..self.arg_count {
      let arg = &self.args[i as usize];

      if arg.candidate
        && i != target_arg_index
        && self.interferes_with_operand(&arg.source, target_arg.target.base)
      {
        return true;
      }
    }

    false
  }
}
