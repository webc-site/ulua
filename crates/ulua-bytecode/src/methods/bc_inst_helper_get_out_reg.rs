use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::bc_inst_helper::BcInstHelper, type_aliases::reg::Reg};

impl BcInstHelper<'_> {
  /// cpp `BcInstHelper::getOutReg()`：指令结果寄存器取自图的 `regs` 映射。
  pub fn get_out_reg(&self) -> Reg {
    let it = self.graph.regs.get(&self.inst);
    LUAU_ASSERT!(it.is_some());
    *it.unwrap()
  }
}
