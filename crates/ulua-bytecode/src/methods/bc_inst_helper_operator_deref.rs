use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{bc_inst::BcInst, bc_inst_helper::BcInstHelper};

impl BcInstHelper<'_> {
  /// cpp `BcInstHelper::operator*`：按下标从持有的图里现取指令，越界即 panic
  /// （旧实现经 `BcRef::operator_arrow` 从 `&Vec<BcInst>` 造 `*mut BcInst`，属 UB）。
  pub fn operator_deref(&self) -> &BcInst {
    LUAU_ASSERT!((self.inst.index as usize) < self.graph.instructions.len());
    &self.graph.instructions[self.inst.index as usize]
  }
}
