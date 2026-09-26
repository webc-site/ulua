use ulua_common::fflag::LuauCodegenPropagateFallbackTags;

use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    const_prop_in_block_chain::const_prop_in_block_chain,
    const_prop_in_fallback::const_prop_in_fallback,
  },
  records::{const_prop_state::ConstPropState, ir_builder::IrBuilder},
};

pub fn const_prop_in_block_chains(build: &mut IrBuilder) {
  // state 门面的 `build` 裸指针在构造语句内即完成交割；此后对 build 的字段解构
  // （function 与 constant_map 互不相交）供各 callee 直接借用，无第二别名。
  let mut state = ConstPropState::const_prop_state_const_prop_state(build);

  let IrBuilder {
    function,
    constant_map,
    ..
  } = build;

  let mut visited = vec![0u8; function.blocks.len()];

  function
    .block_exit_tags
    .resize(function.blocks.len(), Vec::new());

  if LuauCodegenPropagateFallbackTags.get() {
    function
      .fallback_entry_tags
      .resize(function.blocks.len(), Vec::new());
  }

  // 求值时点与拆分前 for 区间头一致（循环前一次）。
  let nblocks = function.blocks.len();

  for i in 0..nblocks {
    let kind = function.blocks[i].kind;

    if kind == IrBlockKind::Dead {
      continue;
    }

    if LuauCodegenPropagateFallbackTags.get() {
      if visited[i] != 0 {
        continue;
      }

      if kind == IrBlockKind::Fallback {
        const_prop_in_fallback(function, constant_map, &mut visited, i as u32, &mut state);
      } else {
        const_prop_in_block_chain(function, constant_map, &mut visited, i as u32, &mut state);
      }
    } else {
      if kind == IrBlockKind::Fallback {
        continue;
      }

      if visited[i] != 0 {
        continue;
      }

      const_prop_in_block_chain(function, constant_map, &mut visited, i as u32, &mut state);
    }
  }
}
