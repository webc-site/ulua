use crate::records::{bc_inst_helper::BcInstHelper, bc_op::BcOp};

impl BcInstHelper<'_> {
  pub fn slice_inputs(&self, start_from: u32) -> Vec<BcOp> {
    let ops = &self.inst.operator_deref().ops;
    let start = start_from as usize;
    if start >= ops.len() {
      Vec::new()
    } else {
      ops[start..].to_vec()
    }
  }
}
