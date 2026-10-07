use crate::{macros::bc_inst_view::bc_inst_view, records::bc_op::BcOp, type_aliases::reg::Reg};

bc_inst_view!(pub BcGetTableKS = LOP_GETTABLEKS, create, from, op, append_to);

impl BcGetTableKS<'_, '_> {
  pub(crate) fn set_source(&mut self, value: BcOp) {
    self.base.set_bc_op(0, value);
  }

  pub(crate) fn set_hint(&mut self, value: u32) {
    self.base.set_imm_input(1, value as i32);
  }

  pub(crate) fn set_key(&mut self, value: u32) {
    self.base.set_vm_const(2, value);
  }

  pub(crate) fn set_out_reg(&mut self, out: Reg) {
    self.base.set_out_reg(out);
  }
}
