use alloc::{vec, vec::Vec};

use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    generate_vm_exit_blocks::generate_vm_exit_blocks, ir::is_pseudo,
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

  // J4r-step2 前置修复：置位与递减拆两阶段。原单遍历（块链逐指令
  // `update_remaining_uses` 内置位+递减）隐含「遍历序中定义先于使用」的拓扑序
  // 假设——多链起点顺序不保证该序：条件边目标块可先于其值定义块被遍历
  // （NAMECALL 链走扩深的 absent→probe 形态即触发 remaining 未置位断言，
  // 诊断轨迹 GetHashNodeAddr 引用后置位的 LoadPointer）。先对全函数（含
  // Fallback/Dead 块——其指令同样被引用计数）置位，块链遍历只做递减，顺序无关。
  for (index, inst) in function.instructions.iter().enumerate() {
    if !is_pseudo(inst.cmd) {
      remaining_uses[index] = inst.use_count as u32;
    }
  }

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
