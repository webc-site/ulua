use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

impl IrBuilder {
  pub fn undef(&mut self) -> IrOp {
    IrOp::ir_op_ir_op_kind_u32(IrOpKind::Undef, 0)
  }
}
