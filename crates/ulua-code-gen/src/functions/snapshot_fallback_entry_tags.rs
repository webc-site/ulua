use alloc::vec::Vec;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  functions::is_pseudo::is_pseudo,
  records::{
    const_prop_state::ConstPropState, ir_data::K_UNKNOWN_TAG, ir_function::IrFunction, ir_op::IrOp,
  },
};

/// cpp `snapshotFallbackEntryTags`：对指令引用的 fallback 块记录/合并入口 tag 快照，
/// 供 constPropInFallback 设置 fallback 块入口状态。
///
/// 目标指令以索引传入：读取 `function.instructions[index]` 的操作数、写
/// `function.fallback_entry_tags` 分属不同字段，借用天然不相交，调用方无需再持有
/// 指令的可变别名。
pub fn snapshot_fallback_entry_tags(function: &mut IrFunction, index: u32, state: &ConstPropState) {
  let inst = &function.instructions[index as usize];
  if is_pseudo(inst.cmd) {
    return;
  }

  // 操作数快照（IrOp 为 Copy）：循环体内同时读写 function 其他字段
  let ops: Vec<IrOp> = inst.ops.as_slice().to_vec();

  for &op in &ops {
    if op.kind() != IrOpKind::Block {
      continue;
    }

    // 索引句柄收口：op 为 Block 操作数时 `block_op`+`get_block_index` 的组合恒退化为
    // op.index() 本身（块指针即 `blocks[idx]` 的地址），直接按写下标访问 Vec，
    // 免去裸指针别名与两次重建借用；越界仍按原语义 panic。
    let block_idx = op.index();

    if function.blocks[block_idx as usize].kind != IrBlockKind::Fallback {
      continue;
    }

    if block_idx as usize >= function.fallback_entry_tags.len() {
      continue;
    }

    let tags = &mut function.fallback_entry_tags[block_idx as usize];
    let max_reg = state.max_reg.max(0) as usize;

    if tags.is_empty() {
      tags.reserve(max_reg + 1);

      for reg_info in &state.regs[..=max_reg] {
        tags.push(reg_info.tag);
      }
    } else {
      // tag 不一致时置为 unknown
      for (i, reg_info) in state.regs[..=max_reg].iter().enumerate() {
        if i >= tags.len() {
          break;
        }

        if tags[i] != reg_info.tag {
          tags[i] = K_UNKNOWN_TAG;
        }
      }

      // 无法识别的 tag 视为 unknown
      let tail_start = (max_reg + 1).min(tags.len());
      for tag_slot in tags[tail_start..].iter_mut() {
        *tag_slot = K_UNKNOWN_TAG;
      }
    }
  }
}
