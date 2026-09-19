use crate::records::{bc_inst_helper::BcInstHelper, bc_op::BcOp};

impl BcInstHelper<'_> {
  /// cpp `BcInstHelper::appendTo(block)`：改写指令的 `block` 并挂到块尾。
  pub fn append_to(&mut self, block: BcOp) {
    let op = self.inst;
    self.operator_deref_mut().block = block;
    self.graph.block_op(block).append_instruction(op);
  }
}
