use alloc::{vec, vec::Vec};

use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    generate_vm_exit_blocks::generate_vm_exit_blocks,
    mark_dead_stores_in_block_chain::mark_dead_stores_in_block_chain,
  },
  records::ir_builder::IrBuilder,
};

pub fn mark_dead_stores_in_block_chains(build: &mut IrBuilder) {
  let num_blocks = build.function.blocks.len();
  let num_insts = build.function.instructions.len();

  let mut visited: Vec<u8> = vec![0u8; num_blocks];
  let mut remaining_uses: Vec<u32> = vec![0u32; num_insts];
  let mut block_idx_chain: Vec<u32> = Vec::new();
  let mut recorded_vm_exit_syncs: Vec<u32> = Vec::new();

  // 入口一次字段解构：`function` 与 `constant_map` 是 IrBuilder 的互不相交字段借用，
  // DSE 全程不再持有 `&mut IrBuilder`，消除 build/build.function 交叠（对齐 const-prop 波）。
  let IrBuilder {
    function,
    constant_map,
    ..
  } = build;

  for block_idx in 0..num_blocks {
    let kind = function.blocks[block_idx].kind;

    if matches!(kind, IrBlockKind::Fallback | IrBlockKind::Dead) {
      continue;
    }

    if visited[block_idx] != 0 {
      continue;
    }

    mark_dead_stores_in_block_chain(
      function,
      constant_map,
      &mut visited,
      &mut remaining_uses,
      &mut block_idx_chain,
      &mut recorded_vm_exit_syncs,
      block_idx as u32,
    );
  }

  generate_vm_exit_blocks(build, &recorded_vm_exit_syncs);
}
