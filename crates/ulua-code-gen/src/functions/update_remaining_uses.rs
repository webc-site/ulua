use crate::{
  enums::ir_op_kind::IrOpKind, macros::codegen_assert::CODEGEN_ASSERT,
  records::remove_dead_store_state::RemoveDeadStoreState, type_aliases::ir::IrOps,
};

/// 指令操作数快照传入（对齐 DSE 的索引化交割）：只做引用计数递减，
/// 不再借用函数数组槽位。遍历与 `visit_arguments` 同语义
/// （pseudo 指令跳过操作数）。
pub fn decrement_remaining_uses(state: &mut RemoveDeadStoreState, ops: &IrOps) {
  // 计数经字段视图访问器派生独占可变借用
  let remaining = state.remaining_uses_mut();

  // 置位已在入口的全函数 place pass 完成（两阶段拆分，见
  // mark_dead_stores_in_block_chains 的 J4r-step2 前置修复注）——本函数只做
  // 块链遍历中的参数递减。

  for op in ops.iter() {
    if op.kind() == IrOpKind::Inst {
      CODEGEN_ASSERT!(remaining[op.index() as usize] != 0);
      remaining[op.index() as usize] -= 1;
    }
  }
}
