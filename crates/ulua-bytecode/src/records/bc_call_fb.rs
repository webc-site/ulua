use std::vec::Vec;

use crate::{macros::bc_inst_view::bc_inst_view, records::bc_op::BcOp};

bc_inst_view!(pub BcCallFB = LOP_CALLFB, from, op);

impl BcCallFB<'_, '_> {
  pub(crate) const K_PARAM_START_INPUT: u32 = 4;

  pub(crate) fn params(&self) -> Vec<BcOp> {
    self.base.slice_inputs(Self::K_PARAM_START_INPUT)
  }

  pub(crate) fn param_count(&mut self) -> i32 {
    self.base.int_imm_input(0)
  }

  pub(crate) fn set_param_count(&mut self, value: u32) {
    self.base.set_imm_input(0, value as i32);
  }

  pub(crate) fn return_count(&mut self) -> i32 {
    self.base.int_imm_input(1)
  }

  pub(crate) fn fb_slot(&mut self) -> i32 {
    self.base.int_imm_input(2)
  }

  pub(crate) fn set_fb_slot(&mut self, value: i32) {
    self.base.set_imm_input(2, value);
  }

  pub(crate) fn target(&mut self) -> BcOp {
    self.base.get_bc_op(3)
  }
}
