use alloc::vec::Vec;

use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::visit_arguments::visit_arguments,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_inst::IrInst, remove_dead_store_state::RemoveDeadStoreState},
};

pub fn update_remaining_uses(state: &mut RemoveDeadStoreState, inst: &mut IrInst, index: u32) {
  let remaining: *mut Vec<u32> = state.remaining_uses;

  unsafe {
    (&mut *remaining)[index as usize] = inst.use_count as u32;
  }

  visit_arguments(inst, |op| {
    if op.kind() == IrOpKind::Inst {
      unsafe {
        CODEGEN_ASSERT!((&*remaining)[op.index() as usize] != 0);
        (&mut *remaining)[op.index() as usize] -= 1;
      }
    }
  });
}
