use ulua_analysis::{
  records::control_flow_graph::ControlFlowGraph, type_aliases::block_id::BlockId,
};

/// `b` 为 `BlockId` u32 句柄（见 ulua-analysis `records::block_registry`），
/// 按 CFG 发射序线性查找其下标；未命中即构建契约被破坏（cpp 同场景以地址
/// 相等查找，语义一致）。
pub fn block_index(cfg: &ControlFlowGraph, b: BlockId) -> usize {
  for (i, block) in cfg.blocks.iter().enumerate() {
    if *block == b {
      return i;
    }
  }

  panic!("block was not found in CFG");
}
