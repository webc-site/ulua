use ulua_common::macros::luau_unlikely::LUAU_UNLIKELY;

use crate::records::{ir_inst::IrInst, ir_op::IrOp};

#[inline]
pub fn get_op_mut(inst: &mut IrInst, idx: u32) -> &mut IrOp {
  if LUAU_UNLIKELY!(idx >= inst.ops.size()) {
    inst.ops.resize(idx + 1);
  }
  &mut inst.ops[idx as usize]
}
