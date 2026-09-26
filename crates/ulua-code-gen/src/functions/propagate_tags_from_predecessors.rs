use ulua_common::fflag::{LuauCodegenPropagateFallbackTags, LuauCodegenSkipDeadPredecessorTags};

use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{predecessors::predecessors, reg_bitset::reg_bit_test},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_data::K_UNKNOWN_TAG, ir_function::IrFunction},
  traits::tag_access::TagAccess,
};

/// cpp `propagateTagsFromPredecessors`：用前驱出口 tag 收敛块入口已知 tag。
/// C++ 以 `std::function` 接收 get/set；此处用 `TagAccess` 静态分发，避免堆分配与裸指针。
/// 目标块以索引传入（cpp 版仅有 `getBlockIndex(block)` 一处用途）。
pub fn propagate_tags_from_predecessors<T: TagAccess + ?Sized>(
  function: &IrFunction,
  block_idx: u32,
  state: &mut T,
) {
  if block_idx >= function.cfg.predecessors_offsets.len() as u32 {
    return;
  }

  // 入口块有隐式的函数入口边，该时刻尚无任何 tag 信息，不得从回边前驱传播
  // （对齐 cpp IrUtils.cpp:1884 `FFlag::LuauCodegenPropagateFallbackTags && entryBlock == blockIdx`）
  if LuauCodegenPropagateFallbackTags.get() && function.entry_block == block_idx {
    return;
  }

  let preds = predecessors(&function.cfg, block_idx);

  if preds.as_slice().is_empty() {
    return;
  }

  let mut min_regs_known: usize = usize::MAX;

  let num_block_exit_tags = function.block_exit_tags.len();

  // cpp 单次取前驱迭代器两轮复用；切片经 BlockIteratorWrapper 的共享借用，两轮一致
  for &pred_idx in preds.as_slice() {
    // 死前驱不参与 tag 传播（对齐 cpp `FFlag::LuauCodegenSkipDeadPredecessorTags`）
    if LuauCodegenSkipDeadPredecessorTags.get()
      && function.blocks[pred_idx as usize].kind == IrBlockKind::Dead
    {
      continue;
    }

    if pred_idx as usize >= num_block_exit_tags {
      return;
    }

    min_regs_known = min_regs_known.min(function.block_exit_tags[pred_idx as usize].len());
  }

  let in_ = &function.cfg.r#in[block_idx as usize];

  let mut first_predecessor = true;

  for &pred_idx in preds.as_slice() {
    if LuauCodegenSkipDeadPredecessorTags.get()
      && function.blocks[pred_idx as usize].kind == IrBlockKind::Dead
    {
      continue;
    }

    let pred_tags = &function.block_exit_tags[pred_idx as usize];

    CODEGEN_ASSERT!(min_regs_known <= pred_tags.len());

    for (i, _) in pred_tags.iter().enumerate().take(min_regs_known) {
      // 只有 live-in 的寄存器能从前驱获得信息
      let live_in =
        reg_bit_test(&in_.regs, i) || (in_.vararg_seq && i >= in_.vararg_start as usize);

      if live_in {
        let current_tag = state.get_tag(i);

        if first_predecessor {
          state.set_tag(i, pred_tags[i]);
        } else if current_tag != K_UNKNOWN_TAG && current_tag != pred_tags[i] {
          state.set_tag(i, K_UNKNOWN_TAG);
        }
      }
    }

    first_predecessor = false;
  }
}
