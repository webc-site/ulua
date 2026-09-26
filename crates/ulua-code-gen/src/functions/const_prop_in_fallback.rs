use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{
    const_prop_in_block::const_prop_in_block, predecessors::predecessors, reg_bitset::reg_bit_test,
    save_block_exit_state::save_block_exit_state,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    const_prop_state::ConstPropState, ir_builder::ConstantMap, ir_data::K_UNKNOWN_TAG,
    ir_function::IrFunction, ir_op::IrOp,
  },
};

/// cpp `constPropInFallback`：所有前驱访问过后，用 fallbackEntryTags 设置
/// fallback 块入口已知 tag，再对该块做常量传播并保存出口状态。
/// 目标块以索引传入；函数与常量表为互斥字段借用，块/指令一律按索引即时访问。
pub fn const_prop_in_fallback(
  function: &mut IrFunction,
  constant_map: &mut ConstantMap,
  visited: &mut [u8],
  block_idx: u32,
  state: &mut ConstPropState,
) {
  if block_idx as usize >= function.cfg.predecessors_offsets.len() {
    return;
  }

  // 必须访问完所有前驱，fallbackEntryTags 信息才正确
  for pred_idx in predecessors(&function.cfg, block_idx) {
    if visited[pred_idx as usize] == 0 {
      return;
    }
  }

  CODEGEN_ASSERT!(visited[block_idx as usize] == 0);
  visited[block_idx as usize] = 1;

  state.clear();

  // fallback_entry_tags/cfg.in 在 const_prop_in_block_chains 已按 blocks.len() resize，
  // block_idx 通过上界检查故界内
  let tags = function.fallback_entry_tags[block_idx as usize].clone();

  // 为 fallback block 设置入口状态
  {
    let in_ = &function.cfg.r#in[block_idx as usize];

    for (i, &tag) in tags.iter().enumerate() {
      if tag == K_UNKNOWN_TAG {
        continue;
      }

      // 只有 live in 寄存器才能记录入口 tag
      let live_in =
        reg_bit_test(&in_.regs, i) || (in_.vararg_seq && i >= in_.vararg_start as usize);

      if live_in {
        // cpp `build.vmReg(i)` 为纯构造函数（不触碰 builder 状态），就地内联为 IrOp
        let op = IrOp::ir_op_kind_u32(IrOpKind::VmReg, i as u32);
        state.update_tag(op, tag);
      }
    }
  }

  const_prop_in_block(function, constant_map, block_idx, state);

  save_block_exit_state(function, block_idx, state);
}
