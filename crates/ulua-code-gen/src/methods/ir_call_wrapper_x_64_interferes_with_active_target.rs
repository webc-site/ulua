use crate::{
  functions::same_underlying_register::same_underlying_register,
  records::{ir_call_wrapper_x_64::IrCallWrapperX64, register_x_64::RegisterX64},
};

impl IrCallWrapperX64 {
  pub fn interferes_with_active_target(&self, source_reg: RegisterX64) -> bool {
    for i in 0..self.arg_count {
      let arg = &self.args[i as usize];

      if arg.candidate && same_underlying_register(arg.target.base, source_reg) {
        return true;
      }
    }

    false
  }
}
