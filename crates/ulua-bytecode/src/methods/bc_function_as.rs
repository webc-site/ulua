use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{bc_function::BcFunction, bc_inst::BcInst, bc_op::BcOp, bc_ref::BcRef};

pub trait BcInstType {
  const OPCODE: i32;
}

impl BcFunction {
  pub fn as_<T>(&self, op: BcOp) -> T
  where
    T: BcInstType + for<'a> From<(&'a BcFunction, BcRef<'a, BcInst>)>,
  {
    let insn = self.inst(op);
    LUAU_ASSERT!(insn.operator_deref().op as i32 == T::OPCODE);

    T::from((self, insn))
  }
}
