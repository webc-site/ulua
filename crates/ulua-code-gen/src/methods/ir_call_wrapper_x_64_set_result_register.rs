use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_call_wrapper_x_64::IrCallWrapperX64, register_x_64::RegisterX64},
};

impl IrCallWrapperX64 {
  pub fn set_result_register(&mut self, reg: RegisterX64, inst_idx: u32) {
    CODEGEN_ASSERT!(!reg.register_x_64_operator_eq(RegisterX64::NOREG));

    self.result_reg = reg;
    self.result_inst_idx = inst_idx;
  }
}
