use crate::{macros::bc_inst_view::bc_inst_view, type_aliases::reg::Reg};

bc_inst_view!(pub(crate) BcLoadNil = LOP_LOADNIL, create, op, prepend_to, append_to);

impl BcLoadNil<'_, '_> {
  pub(crate) fn set_out_reg(&mut self, out: Reg) {
    self.base.set_out_reg(out);
  }
}
