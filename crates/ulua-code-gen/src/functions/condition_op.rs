use core::mem::transmute;

use crate::{
  enums::{ir_condition::IrCondition, ir_op_kind::IrOpKind},
  records::ir_op::IrOp,
};

pub fn condition_op(op: IrOp) -> IrCondition {
  debug_assert!(op.kind() == IrOpKind::Condition);
  let index = op.index();
  if index < IrCondition::Count as u32 {
    unsafe { transmute::<u8, IrCondition>(index as u8) }
  } else {
    IrCondition::Count
  }
}
