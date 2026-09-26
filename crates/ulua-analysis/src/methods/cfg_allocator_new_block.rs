//! Source: `Analysis/src/ControlFlowGraph.cpp:102-105` (hand-ported)
//! C++ `Block* CFGAllocator::newBlock(BlockKind kind, std::string debugName)`.
use alloc::string::String;

use crate::{
  enums::block_kind::BlockKind,
  records::{block::Block, block_registry::register_block, cfg_allocator::CfgAllocator},
  type_aliases::block_id::BlockId,
};

impl CfgAllocator {
  pub fn new_block(&mut self, kind: BlockKind, debug_name: String) -> BlockId {
    // C++: return block.allocate(kind, debugName);
    // TypedAllocator::allocate takes the constructed value; build the Block
    // in place (C++ constructs `T{args...}` inside allocate).
    // 裸指针仅在注册点出现：arena 地址经 `register_block` 换成 u32 句柄，
    // 之后全链（preds/succs、builder 集合、转储）只持有/比较句柄
    // （见 `records::block_registry` 模块契约）。
    register_block(self.block.allocate(Block::new(kind, debug_name)))
  }
}
