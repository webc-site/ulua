use crate::{
  functions::mark_dead_stores_in_inst::mark_dead_stores_in_inst,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_block::K_BLOCK_FLAG_SAFE_ENV_CHECK, ir_builder::ConstantMap, ir_function::IrFunction,
    remove_dead_store_state::RemoveDeadStoreState,
  },
};

/// 目标块以 (function, block_idx) 传入：块头快照与逐指令回调都直接对函数参数
/// 短借用派生，函数不再经 `*mut` 就地交割，块/指令槽位取址的 `addr_of_mut!`
/// 全部消失（对齐 const-prop 波的索引化 + 字段解构模式）。
pub fn mark_dead_stores_in_block(
  function: &mut IrFunction,
  constant_map: &mut ConstantMap,
  block_idx: u32,
  state: &mut RemoveDeadStoreState,
) {
  // Block 可能在开头就建立起安全环境，也可能触发 VM exit
  let (start, finish, safe_env_check) = {
    let block = &function.blocks[block_idx as usize];
    (
      block.start,
      block.finish,
      block.flags & K_BLOCK_FLAG_SAFE_ENV_CHECK != 0,
    )
  };

  if safe_env_check {
    state.read_all_regs();
  }

  for index in start..=finish {
    CODEGEN_ASSERT!((index as usize) < function.instructions.len());

    mark_dead_stores_in_inst(state, constant_map, function, block_idx, index);
  }
}
