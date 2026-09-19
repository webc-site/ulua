use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_op::BcOp, call_inliner::CallInliner},
};

impl<'a> CallInliner<'a> {
  /// cpp `setReturnOp(idx, op)`：第 `idx` 个返回槽位累计多个候选时折叠成 phi。
  pub fn set_return_op(&mut self, idx: u32, op: BcOp) {
    if (idx as usize) >= self.return_ops.len() {
      self.return_ops.resize(idx as usize + 1, BcOp::new());
    }

    if self.return_ops[idx as usize].kind == BcOpKind::None {
      self.return_ops[idx as usize] = op;
      return;
    }

    if self.return_ops[idx as usize].kind != BcOpKind::Phi {
      let phi_op = self.caller.add_phi();
      let previous = self.return_ops[idx as usize];
      self.caller.phi_op(phi_op).ops.push_back(previous);
      self.return_ops[idx as usize] = phi_op;
    } else {
      let phi_op = self.return_ops[idx as usize];
      self.caller.phi_op(phi_op).ops.push_back(op);
    }
  }
}
