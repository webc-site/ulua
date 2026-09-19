use crate::records::{bc_inst_helper::BcInstHelper, bc_op::BcOp};

impl BcInstHelper<'_> {
  /// cpp `BcInstHelper::prependTo(block)`：改写指令的 `block` 并挂到块首。
  pub fn prepend_to(&mut self, block: BcOp) {
    let op = self.inst;
    self.operator_deref_mut().block = block;
    self.graph.block_op(block).ops.push_front(op);
  }
}
