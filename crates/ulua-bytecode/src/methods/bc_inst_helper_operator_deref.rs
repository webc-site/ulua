use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{bc_inst::BcInst, bc_inst_helper::BcInstHelper};

impl<'a> BcInstHelper<'a> {
  pub fn operator_deref(&self) -> &BcInst {
    LUAU_ASSERT!((self.inst.op.index as usize) < self.inst.vec.len());
    &self.inst.vec[self.inst.op.index as usize]
  }
}
