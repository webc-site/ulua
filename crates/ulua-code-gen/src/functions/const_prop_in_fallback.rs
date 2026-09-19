use crate::{
  functions::{
    const_prop_in_block::const_prop_in_block, predecessors::predecessors,
    save_block_exit_state::save_block_exit_state,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    const_prop_state::ConstPropState, ir_block::IrBlock, ir_builder::IrBuilder,
    ir_data::K_UNKNOWN_TAG, ir_function::IrFunction,
  },
};

/// cpp `constPropInFallback`：所有前驱访问过后，用 fallbackEntryTags 设置
/// fallback 块入口已知 tag，再对该块做常量传播并保存出口状态。
///
/// # Safety
/// `build.function` 以裸指针访问，调用方须保证 `block` 指向 `build.function.blocks`
/// 中的有效块，且全程独占访问。
pub unsafe fn const_prop_in_fallback(
  build: &mut IrBuilder,
  visited: &mut [u8],
  block: &mut IrBlock,
  state: &mut ConstPropState,
) {
  let function: *mut IrFunction = &mut build.function;

  let block_idx = unsafe { (*function).get_block_index(&*block) };

  if block_idx as usize >= unsafe { (*function).cfg.predecessors_offsets.len() } {
    return;
  }

  // All predecessors must be visited for fallbackEntryTags info to be correct
  for pred_idx in unsafe { predecessors(&(*function).cfg, block_idx) } {
    if visited[pred_idx as usize] == 0 {
      return;
    }
  }

  CODEGEN_ASSERT!(visited[block_idx as usize] == 0);
  visited[block_idx as usize] = 1;

  state.clear();

  let tags = unsafe { (&(*function).fallback_entry_tags)[block_idx as usize].clone() };
  let in_ = unsafe { &(&(*function).cfg.r#in)[block_idx as usize] };

  // Setup entry state for the fallback block
  for (i, &tag) in tags.iter().enumerate() {
    if tag == K_UNKNOWN_TAG {
      continue;
    }

    // Only live in registers can have entry tags recorded
    let live_in = (in_.regs[i / 64] & (1u64 << (i % 64))) != 0
      || (in_.vararg_seq && i >= in_.vararg_start as usize);

    if live_in {
      let op = build.vm_reg(i as u8);
      state.update_tag(op, tag);
    }
  }

  const_prop_in_block(build, block, state);

  unsafe {
    save_block_exit_state(&mut *function, block, state);
  }
}
