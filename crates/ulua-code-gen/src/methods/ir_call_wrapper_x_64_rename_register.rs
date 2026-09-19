use crate::{
  functions::same_underlying_register::same_underlying_register,
  records::{ir_call_wrapper_x_64::IrCallWrapperX64, register_x_64::RegisterX64},
};

impl IrCallWrapperX64 {
  pub fn rename_register(
    &mut self,
    target: &mut RegisterX64,
    reg: RegisterX64,
    replacement: RegisterX64,
  ) {
    if same_underlying_register(*target, reg) {
      self.add_register_use(replacement);
      self.remove_register_use(*target);

      // Only change index, size is preserved
      let replacement_index = replacement.index();
      *target = RegisterX64 {
        bits: (target.bits & RegisterX64::SIZE_MASK)
          | (replacement_index << RegisterX64::INDEX_SHIFT),
      };
    }
  }
}
