use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_inst_helper::BcInstHelper, bc_op::BcOp},
};

impl BcInstHelper<'_> {
  /// cpp `BcInstHelper::detach()`（BytecodeOps.h:64-71）：把指令从所属块的 `ops`
  /// 序列里摘掉并清空 `block`，指令本身仍留在 `instructions` 池中。
  pub fn detach(&mut self) {
    let block = self.operator_deref().block;
    if block.kind != BcOpKind::Block {
      return;
    }
    let op = self.inst;
    self
      .graph
      .block_op(block)
      .ops
      .retain(|existing| *existing != op);
    self.operator_deref_mut().block = BcOp::new();
  }
}
