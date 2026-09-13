use crate::records::{bc_inst_helper::BcInstHelper, bc_op::BcOp};

impl BcInstHelper<'_> {
  pub fn get_bc_op(&mut self, input_idx: u32) -> BcOp {
    if (input_idx as usize) >= self.operator_deref().ops.len() {
      self.operator_deref_mut().ops.resize(input_idx + 1);
    }
    self.operator_deref().ops[input_idx as usize]
  }
}
