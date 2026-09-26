use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::is_pseudo::is_pseudo,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::remove_dead_store_state::RemoveDeadStoreState,
  type_aliases::ir_ops::IrOps,
};

/// 指令数据以 (cmd, use_count, ops) 快照传入（对齐 DSE 的索引化交割）：
/// 只写计数侧，不再借用函数数组槽位。遍历与 `visit_arguments` 同语义
/// （pseudo 指令跳过操作数）。
pub fn update_remaining_uses(
  state: &mut RemoveDeadStoreState,
  index: u32,
  cmd: IrCmd,
  use_count: u16,
  ops: &IrOps,
) {
  // 计数经字段视图访问器派生独占可变借用
  let remaining = state.remaining_uses_mut();

  remaining[index as usize] = use_count as u32;

  if is_pseudo(cmd) {
    return;
  }

  for op in ops.iter() {
    if op.kind() == IrOpKind::Inst {
      CODEGEN_ASSERT!(remaining[op.index() as usize] != 0);
      remaining[op.index() as usize] -= 1;
    }
  }
}
