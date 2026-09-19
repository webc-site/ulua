use crate::records::{call_argument::CallArgument, ir_call_wrapper_x_64::IrCallWrapperX64};

impl IrCallWrapperX64 {
  #[inline]
  pub fn interferes_with_active_sources(
    &self,
    target_arg: &CallArgument,
    target_arg_index: i32,
  ) -> bool {
    // 跳过自身、命中即短路
    self
      .args
      .iter()
      .take(self.arg_count as usize)
      .enumerate()
      .any(|(i, arg)| {
        arg.candidate
          && i as i32 != target_arg_index
          && self.interferes_with_operand(&arg.source, target_arg.target.base)
      })
  }
}
