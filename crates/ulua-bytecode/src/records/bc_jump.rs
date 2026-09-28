use crate::{macros::bc_inst_view::bc_inst_view, records::bc_op::BcOp};

bc_inst_view!(pub(crate) BcJump = LOP_JUMP, create, append_to);

impl BcJump<'_, '_> {
  pub(crate) fn set_target(&mut self, block: BcOp) {
    self.base.set_bc_op(0, block);
  }
}
