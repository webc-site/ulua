use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    compute_cfg_block_edges::compute_cfg_block_edges,
    compute_cfg_dominance_tree_children::compute_cfg_dominance_tree_children,
    compute_cfg_immediate_dominators::compute_cfg_immediate_dominators,
    compute_cfg_live_in_out_reg_sets::compute_cfg_live_in_out_reg_sets,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    block_iterator_wrapper::BlockIteratorWrapper, cfg_info::CfgInfo, ir_block::IrBlock,
    ir_function::IrFunction,
  },
};

pub fn predecessors(cfg: &CfgInfo, block_idx: u32) -> BlockIteratorWrapper<'_> {
  CODEGEN_ASSERT!(block_idx < cfg.predecessors_offsets.len() as u32);

  let start = cfg.predecessors_offsets[block_idx as usize];
  let end = if block_idx + 1 < cfg.predecessors_offsets.len() as u32 {
    cfg.predecessors_offsets[(block_idx + 1) as usize]
  } else {
    cfg.predecessors.len() as u32
  };

  BlockIteratorWrapper::new(&cfg.predecessors[start as usize..end as usize])
}

pub fn successors(cfg: &CfgInfo, block_idx: u32) -> BlockIteratorWrapper<'_> {
  // 行为与 C++ CODEGEN_ASSERT 保持一致，同时避免当前
  // 宏在 ulua_common::functions::assert_call_handler::assert_call_handler 上的类型不匹配。
  assert!(block_idx < cfg.successors_offsets.len() as u32);

  let start = cfg.successors_offsets[block_idx as usize];
  let end = if block_idx + 1 < cfg.successors_offsets.len() as u32 {
    cfg.successors_offsets[(block_idx + 1) as usize]
  } else {
    cfg.successors.len() as u32
  };

  BlockIteratorWrapper::new(&cfg.successors[start as usize..end as usize])
}

pub fn dom_children(cfg: &CfgInfo, block_idx: u32) -> BlockIteratorWrapper<'_> {
  if !(block_idx < cfg.dom_children_offsets.len() as u32) {
    CODEGEN_ASSERT!(false);
  }

  let start = cfg.dom_children_offsets[block_idx as usize];
  let end = if (block_idx + 1) < cfg.dom_children_offsets.len() as u32 {
    cfg.dom_children_offsets[(block_idx + 1) as usize]
  } else {
    cfg.dom_children.len() as u32
  };

  BlockIteratorWrapper::new(&cfg.dom_children[start as usize..end as usize])
}

/// 返回排序表中第 i 块之后的首个非死块索引（None 表示不存在）。
/// 原形态返回 `&mut IrBlock`（含哨兵 dummy 块），调用方仅用于与
/// `expected_next_block` 的一致性断言，索引形态即其等价物。
pub fn get_next_block(function: &IrFunction, sorted_blocks: &[u32], i: usize) -> Option<u32> {
  sorted_blocks
    .iter()
    .skip(i + 1)
    .map(|&idx| idx as usize)
    .find(|&idx| function.blocks[idx].kind != IrBlockKind::Dead)
    .map(|idx| idx as u32)
}

pub fn is_entry_block(block: &IrBlock) -> bool {
  block.use_count == 0 && block.kind != IrBlockKind::Dead
}

pub fn get_block_kind_name(kind: IrBlockKind) -> &'static str {
  match kind {
    IrBlockKind::Bytecode => "bb_bytecode",
    IrBlockKind::Fallback => "bb_fallback",
    IrBlockKind::Internal => "bb",
    IrBlockKind::Linearized => "bb_linear",
    IrBlockKind::ExitSync => "bb_exit",
    IrBlockKind::Dead => "dead",
  }
}

pub fn compute_cfg_info(function: &mut IrFunction) {
  compute_cfg_block_edges(function);
  compute_cfg_immediate_dominators(function);
  compute_cfg_dominance_tree_children(function);
  compute_cfg_live_in_out_reg_sets(function);
}
