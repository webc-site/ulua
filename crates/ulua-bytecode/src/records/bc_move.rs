use crate::{macros::bc_inst_view::bc_inst_view, records::bc_op::BcOp, type_aliases::reg::Reg};

bc_inst_view!(pub(crate) BcMove = LOP_MOVE, create, op, append_to);

impl BcMove<'_, '_> {
  pub(crate) fn set_src(&mut self, value: BcOp) {
    self.base.set_bc_op(0, value);
  }

  pub(crate) fn set_out_reg(&mut self, out: Reg) {
    self.base.set_out_reg(out);
  }
}
