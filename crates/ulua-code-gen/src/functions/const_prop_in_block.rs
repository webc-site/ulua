use ulua_common::fflag::LuauCodegenPropagateFallbackTags;

use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    apply_substitutions_ir_utils::apply_substitutions_at, const_prop_in_inst::const_prop_in_inst,
    fold_constants::fold_constants, is_pseudo::is_pseudo,
    snapshot_fallback_entry_tags::snapshot_fallback_entry_tags,
  },
  records::{
    const_prop_state::ConstPropState, ir_block::K_BLOCK_FLAG_SAFE_ENV_CHECK,
    ir_builder::ConstantMap, ir_function::IrFunction,
  },
};

/// 块与指令一律以索引定位：函数、常量表（constant_map）为互不相交的字段借用，
/// 与 ConstPropState 门面（内部经 build.function 派生视图）顺序交割，无裸指针别名。
pub fn const_prop_in_block(
  function: &mut IrFunction,
  constant_map: &mut ConstantMap,
  block_idx: u32,
  state: &mut ConstPropState,
) {
  // 块区间边界快照一次（cpp 亦在循环头取 block->start/finish；块可被 kill 后
  // block 数据不保证稳定，边界保持快照语义），kind 每轮经索引即时读取
  let (block_start, block_finish, block_flags) = {
    let block = &function.blocks[block_idx as usize];
    (block.start, block.finish, block.flags)
  };

  if (block_flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
    state.in_safe_env = true;
  }

  for index in block_start..=block_finish {
    apply_substitutions_at(function, index);
    fold_constants(function, constant_map, block_idx, index);

    // fflag 读数与 inst.cmd 读数均为纯读，短路求值次序与拆分前一致。
    if LuauCodegenPropagateFallbackTags.get()
      && !is_pseudo(function.instructions[index as usize].cmd)
    {
      snapshot_fallback_entry_tags(function, index, state);
    }

    const_prop_in_inst(state, function, constant_map, block_idx, index);

    if function.blocks[block_idx as usize].kind == IrBlockKind::Dead {
      break;
    }
  }
}
