use std::vec::Vec;

use crate::{macros::bc_inst_view::bc_inst_view, records::bc_op::BcOp};

bc_inst_view!(pub(crate) BcReturn = LOP_RETURN, from);

impl BcReturn<'_, '_> {
  pub(crate) const K_VALUES_START_INPUT: u32 = 1;

  pub(crate) fn return_count(&mut self) -> i32 {
    self.base.int_imm_input(0)
  }

  pub(crate) fn set_return_count(&mut self, value: u32) {
    self.base.set_imm_input(0, value as i32);
  }

  pub fn values(&mut self) -> Vec<BcOp> {
    if self.return_count() == 0 {
      Vec::new()
    } else {
      self.base.slice_inputs(Self::K_VALUES_START_INPUT)
    }
  }
}
