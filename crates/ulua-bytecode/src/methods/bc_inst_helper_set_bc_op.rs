use crate::records::{bc_inst::BcInst, bc_inst_helper::BcInstHelper, bc_op::BcOp};

impl BcInstHelper<'_> {
  pub(crate) fn set_bc_op(&mut self, input_idx: u32, op: BcOp) {
    if input_idx >= self.operator_deref().ops.len() as u32 {
      self.operator_deref_mut().ops.resize(input_idx + 1);
    }
    self.operator_deref_mut().ops[input_idx as usize] = op;
  }

  pub(crate) fn operator_deref_mut(&mut self) -> &mut BcInst {
    unsafe { &mut *self.inst.operator_arrow() }
  }
}
