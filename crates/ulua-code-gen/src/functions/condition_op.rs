use crate::{
  enums::{ir_condition::IrCondition, ir_op_kind::IrOpKind},
  records::ir_op::IrOp,
};

pub fn condition_op(op: IrOp) -> IrCondition {
  debug_assert!(op.kind() == IrOpKind::Condition);
  // 越界判别值钳制为 Count（与原实现一致）；界内经类型化静态表转换，替代裸 transmute
  IrCondition::from_discriminant(op.index().try_into().unwrap_or(u8::MAX))
    .unwrap_or(IrCondition::Count)
}
