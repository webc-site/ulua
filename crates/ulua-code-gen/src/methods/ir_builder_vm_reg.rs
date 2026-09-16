use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

impl IrBuilder {
  pub fn vm_reg(&mut self, index: u8) -> IrOp {
    IrOp::ir_op_kind_u32(IrOpKind::VmReg, index as u32)
  }
}
