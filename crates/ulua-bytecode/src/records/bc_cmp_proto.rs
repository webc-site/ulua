use crate::{macros::bc_inst_view::bc_inst_view, records::bc_op::BcOp};

bc_inst_view!(pub(crate) BcCmpProto = LOP_CMPPROTO, create, append_to);

impl BcCmpProto<'_, '_> {
  pub(crate) fn set_closure(&mut self, value: BcOp) {
    self.base.set_bc_op(0, value);
  }

  pub(crate) fn set_proto_id(&mut self, value: u32) {
    self.base.set_imm_input(1, value as i32);
  }

  pub(crate) fn set_fallback(&mut self, value: BcOp) {
    self.base.set_bc_op(2, value);
  }
}
