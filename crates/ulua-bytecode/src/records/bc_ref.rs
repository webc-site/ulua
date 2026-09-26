// （b）镜像定形：本件内 cpp 同形访问器/类型全仓零消费，为免降级触发 dead_code 升级而保持 pub；
// 非降级对象，勿删（批次账 b28-bc-rt-tail）。
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::bc_op::BcOp;

/// cpp `BcRef<T>`（`BytecodeGraph.h`：`const Vec<T>& vec` + `BcOp op`）——
/// **只读**视图：仅暴露 `operator_deref` 取 `&T`。可变访问必须由持有
/// `&mut BcFunction` 的一方经 `block_op` / `inst_op` / `phi_op` / `proj_op` /
/// `imm_op` / `const_op` 完成，不从共享借用上伪造 `*mut T`（那是 UB）。
#[derive(Debug)]
pub struct BcRef<'a, T> {
  pub(crate) vec: &'a Vec<T>,
  pub(crate) op: BcOp,
}

impl<'a, T> Clone for BcRef<'a, T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<'a, T> Copy for BcRef<'a, T> {}

impl<'a, T> BcRef<'a, T> {
  pub fn operator_deref(&self) -> &T {
    LUAU_ASSERT!((self.op.index as usize) < self.vec.len());
    &self.vec[self.op.index as usize]
  }
}
