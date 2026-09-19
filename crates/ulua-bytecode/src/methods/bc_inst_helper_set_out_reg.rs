use crate::{records::bc_inst_helper::BcInstHelper, type_aliases::reg::Reg};

impl BcInstHelper<'_> {
  /// cpp `BcInstHelper::setOutReg(reg)`。
  pub fn set_out_reg(&mut self, out: Reg) {
    self.graph.regs.insert(self.inst, out);
  }
}
