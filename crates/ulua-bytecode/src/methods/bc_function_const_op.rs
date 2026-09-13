use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_op::BcOp, bc_vm_const::BcVmConst},
};

impl BcFunction {
  pub fn const_op(&mut self, op: BcOp) -> &mut BcVmConst {
    LUAU_ASSERT!(op.kind == BcOpKind::VmConst);
    &mut self.constants[op.index as usize]
  }
}
