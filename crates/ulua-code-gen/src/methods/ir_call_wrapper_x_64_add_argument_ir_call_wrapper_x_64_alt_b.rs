use crate::{
  enums::size_x_64::SizeX64,
  records::{
    ir_call_wrapper_x_64::IrCallWrapperX64, ir_op::IrOp, operand_x_64::OperandX64,
    scoped_reg_x_64::ScopedRegX64,
  },
};

impl IrCallWrapperX64 {
  pub fn add_argument_size_x_64_scoped_reg_x_64(
    &mut self,
    target_size: SizeX64,
    scoped_reg: &mut ScopedRegX64,
  ) {
    let source = scoped_reg.release();
    self.add_argument_size_x_64_operand_x_64_ir_op(
      target_size,
      OperandX64::from(source),
      IrOp::new(),
    );
  }
}
