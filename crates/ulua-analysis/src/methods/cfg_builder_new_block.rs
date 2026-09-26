//! Source: `Analysis/src/ControlFlowGraph.cpp:245-251` (hand-ported)
//! C++ `Block* CFGBuilder::newBlock(BlockKind kind, std::string debugName, Block* pred)`.
use alloc::string::String;

use crate::{
  enums::block_kind::BlockKind, records::cfg_builder::CfgBuilder, type_aliases::block_id::BlockId,
};

impl CfgBuilder {
  /// C++ default arg `Block* pred = nullptr`；缺省在前 §2 化为
  /// `Option<BlockId>`（不再是 `null_mut()` 哨兵）。
  pub fn new_block(
    &mut self,
    kind: BlockKind,
    debug_name: String,
    pred: Option<BlockId>,
  ) -> BlockId {
    // C++:
    //   Block* b = cfg->newBlock(kind, debugName);
    //   if (pred) pred->addSuccessor(b);
    //   return b;
    // 不变式：`CfgBuilder::new` 构造期置入 cfg 且 lowering 期间从不 take/置
    // None（与 make_cfg 的 take 只在 lowering 结束后发生一次相互印证）。
    let b = self
      .cfg
      .as_mut()
      .expect("构造期置入 cfg，lowering 期间恒 Some")
      .new_block(kind, debug_name);
    if let Some(pred) = pred {
      pred.add_successor(b);
    }
    b
  }
}
