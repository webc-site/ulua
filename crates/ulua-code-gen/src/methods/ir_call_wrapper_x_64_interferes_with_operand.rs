use crate::{
  functions::same_underlying_register::same_underlying_register,
  records::{
    ir_call_wrapper_x_64::IrCallWrapperX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl IrCallWrapperX64 {
  #[inline]
  pub fn interferes_with_operand(&self, op: &OperandX64, reg: RegisterX64) -> bool {
    same_underlying_register(op.base, reg) || same_underlying_register(op.index, reg)
  }
}
