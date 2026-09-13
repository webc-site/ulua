use crate::records::{ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, operand_x_64::OperandX64};

impl IrLoweringX64 {
  pub fn store_double_as_float(&mut self, _dst: OperandX64, _src: IrOp) {
    panic!("ir_lowering_x_64_store_double_as_float not yet translated");
  }
}
