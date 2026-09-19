use crate::records::{bc_op::BcOp, call_inliner::CallInliner};

impl<'a> CallInliner<'a> {
  /// cpp `call->block`：被内联 CALLFB 指令所在块。
  pub(crate) fn call_block_op(&mut self) -> BcOp {
    self.caller.inst_op(self.call_op).block
  }
}
