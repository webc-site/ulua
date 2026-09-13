//! Source: `Analysis/src/ControlFlowGraph.cpp:245-251` (hand-ported)
//! C++ `Block* CFGBuilder::newBlock(BlockKind kind, std::string debugName, Block* pred)`.
use alloc::string::String;

use crate::{
  enums::block_kind::BlockKind,
  records::{block::Block, cfg_builder::CfgBuilder},
};

impl CfgBuilder {
  /// C++ default arg `Block* pred = nullptr`; callers pass `null_mut()` to omit.
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn new_block(
    &mut self,
    kind: BlockKind,
    debug_name: String,
    pred: *mut Block,
  ) -> *mut Block {
    // C++:
    //   Block* b = cfg->newBlock(kind, debugName);
    //   if (pred) pred->addSuccessor(b);
    //   return b;
    let b = self.cfg.as_mut().unwrap().new_block(kind, debug_name);
    if !pred.is_null() {
      unsafe { (*pred).add_successor(b) };
    }
    b
  }
}
