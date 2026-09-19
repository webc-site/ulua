use crate::{
  enums::bc_op_kind::BcOpKind::Proj,
  records::{bc_op::BcOp, call_inliner::CallInliner},
};

impl<'a> CallInliner<'a> {
  /// cpp `findTargetCallProjections()`：为直接引用被内联 CALLFB 的投影建聚合 phi。
  pub fn find_target_call_projections(&mut self) {
    for i in 0..self.caller.projections.len() {
      let proj = self.caller.projections[i];
      if proj.op == self.call_op {
        let proj_op = BcOp::bc_op_bc_op_kind_u32(Proj, i as u32);
        if self.call_projections.contains(&proj_op) {
          continue;
        }
        if (proj.index as usize) >= self.return_ops.len() {
          self.return_ops.resize(proj.index as usize + 1, BcOp::new());
        }
        let phi_op = self.caller.add_phi();
        self.caller.phi_op(phi_op).ops.push_back(proj_op);
        self.call_projections.insert(proj_op);
        self.return_ops[proj.index as usize] = phi_op;
      }
    }
  }
}
