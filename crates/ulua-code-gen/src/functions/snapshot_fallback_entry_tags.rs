use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  functions::is_pseudo::is_pseudo,
  records::{
    const_prop_state::ConstPropState, ir_block::IrBlock, ir_function::IrFunction, ir_inst::IrInst,
  },
};

// IrData.h: `constexpr uint8_t kUnknownTag = 0xff;`
const K_UNKNOWN_TAG: u8 = 0xff;

/// cpp `snapshotFallbackEntryTags`：对指令引用的 fallback 块记录/合并入口 tag 快照，
/// 供 constPropInFallback 设置 fallback 块入口状态。
pub fn snapshot_fallback_entry_tags(
  function: &mut IrFunction,
  inst: &IrInst,
  state: &ConstPropState,
) {
  if is_pseudo(inst.cmd) {
    return;
  }

  for &op in inst.ops.as_slice() {
    if op.kind() != IrOpKind::Block {
      continue;
    }

    let block: *mut IrBlock = function.block_op(op);
    if unsafe { (*block).kind } != IrBlockKind::Fallback {
      continue;
    }

    let block_idx = unsafe { function.get_block_index(&*block) };

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
      // Disagreeing tags result make them unknown
      for (i, reg_info) in state.regs[..=max_reg].iter().enumerate() {
        if i >= tags.len() {
          break;
        }

        if tags[i] != reg_info.tag {
          tags[i] = K_UNKNOWN_TAG;
        }
      }

      // Tags we don't know about are unknown
      let tail_start = (max_reg + 1).min(tags.len());
      for tag_slot in tags[tail_start..].iter_mut() {
        *tag_slot = K_UNKNOWN_TAG;
      }
    }
  }
}
