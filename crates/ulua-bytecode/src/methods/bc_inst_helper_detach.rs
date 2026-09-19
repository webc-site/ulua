use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_inst_helper::BcInstHelper, bc_op::BcOp},
};

impl BcInstHelper<'_> {
  /// cpp `BcInstHelper::detach()`（BytecodeOps.h:64-71）：把指令从所属块的 `ops`
  /// 序列里摘掉并清空 `block`，指令本身仍留在 `instructions` 池中。
  pub fn detach(&mut self) {
    let block = unsafe { (*self.inst.operator_arrow()).block };
    if block.kind != BcOpKind::Block {
      return;
    }
    let op = self.inst.op;
    self
      .graph
      .block_op(block)
      .ops
      .retain(|existing| *existing != op);
    unsafe {
      (*self.inst.operator_arrow()).block = BcOp::new();
    }
  }
}
