use std::vec::Vec;

use crate::{macros::bc_inst_view::bc_inst_view, records::bc_op::BcOp};

bc_inst_view!(pub BcCall = LOP_CALL, from);

impl BcCall<'_, '_> {
  pub(crate) const K_PARAM_START_INPUT: u32 = 3;

  pub(crate) fn params(&self) -> Vec<BcOp> {
    self.base.slice_inputs(Self::K_PARAM_START_INPUT)
  }

  pub(crate) fn set_param_count(&mut self, value: u32) {
    self.base.set_imm_input(0, value as i32);
  }
}
