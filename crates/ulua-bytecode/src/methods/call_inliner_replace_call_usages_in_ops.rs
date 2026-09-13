use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{bc_proj::BcProj, call_inliner::CallInliner},
  type_aliases::bc_ops::BcOps,
};

impl<'a> CallInliner<'a> {
  pub fn replace_call_usages_in_ops(&mut self, ops: &mut BcOps) {
    for op in ops.iter_mut() {
      if let Some(proj_op) = self.call_projections.get(op) {
        let proj: &mut BcProj = self.caller.proj_op(*proj_op);
        LUAU_ASSERT!((proj.index as usize) < self.return_ops.len());
        *op = self.return_ops[proj.index as usize];
      }
    }
  }
}
