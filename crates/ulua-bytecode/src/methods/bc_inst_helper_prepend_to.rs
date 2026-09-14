use crate::records::{bc_inst_helper::BcInstHelper, bc_op::BcOp};

impl BcInstHelper<'_> {
  pub fn prepend_to(&mut self, block: BcOp) {
    unsafe {
      (*self.inst.operator_arrow()).block = block;
    }
    let block_mut = self.graph.block_op(block);
    block_mut.ops.push_front(self.inst.op);
  }
}
