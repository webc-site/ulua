use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

impl IrBuilder {
  pub fn vm_const(&mut self, index: u32) -> IrOp {
    IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmConst, index)
  }
}
