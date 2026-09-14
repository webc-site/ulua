use crate::{
  enums::{ir_condition::IrCondition, ir_op_kind::IrOpKind},
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};
impl IrBuilder {
  pub fn cond(&mut self, cond: IrCondition) -> IrOp {
    IrOp::ir_op_ir_op_kind_u32(IrOpKind::Condition, cond as u8 as u32)
  }
}
