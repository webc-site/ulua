use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{bc_inst::BcInst, bc_inst_helper::BcInstHelper};

impl BcInstHelper<'_> {
  /// cpp `BcInstHelper::operator->`（`BytecodeOps.h`）的可变面：可变访问统一经持有
  /// `&mut BcFunction` 的 `self.graph`（`BcFunction::inst_op`）现取现用，下标越界时
  /// 是边界检查失败的可诊断 panic。
  ///
  /// 旧实现写在 `bc_inst_helper_set_vm_const.rs` 文件底部的 `impl BcRef` 里，对 `u32`
  /// 索引不做边界检查，直接 `&mut *(vec.as_ptr().add(index))` 越界写；现独立成文件
  /// 并改走边界检查路径。
  pub(crate) fn operator_deref_mut(&mut self) -> &mut BcInst {
    LUAU_ASSERT!((self.inst.index as usize) < self.graph.instructions.len());
    self.graph.inst_op(self.inst)
  }
}
