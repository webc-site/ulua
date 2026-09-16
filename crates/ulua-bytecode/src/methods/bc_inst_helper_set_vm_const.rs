use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_inst_helper::BcInstHelper, bc_op::BcOp, bc_ref::BcRef},
};

impl BcInstHelper<'_> {
  pub fn set_vm_const(&mut self, input_idx: u32, cid: u32) {
    LUAU_ASSERT!(cid < self.graph.constants.len() as u32);
    self.set_bc_op(
      input_idx,
      BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmConst, cid),
    );
  }
}

impl<'a, T> BcRef<'a, T> {
  pub(crate) fn operator_deref_mut(&mut self) -> &mut T {
    unsafe { &mut *(self.vec.as_ptr().add(self.op.index as usize) as *mut T) }
  }
}
