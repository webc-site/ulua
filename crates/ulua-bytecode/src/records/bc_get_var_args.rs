use crate::{macros::bc_inst_view::bc_inst_view, type_aliases::reg::Reg};

bc_inst_view!(pub(crate) BcGetVarArgs = LOP_GETVARARGS, from);

impl BcGetVarArgs<'_, '_> {
  pub(crate) const K_START_REG_INPUT: u32 = 0;

  pub(crate) fn values_count(&mut self) -> i32 {
    self.base.int_imm_input(1)
  }

  pub(crate) fn start_reg(&self) -> Reg {
    self.base.operator_deref().ops[Self::K_START_REG_INPUT as usize].index as Reg
  }
}
