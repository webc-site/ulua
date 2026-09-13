//! Source: `Analysis/include/Luau/ControlFlowGraph.h:348-353` (hand-ported)
//! C++ `CFGBuilder::BlockScope::BlockScope(CFGBuilder& builder, Block* target)`.
use crate::records::{block::Block, cfg_builder::CfgBuilder};

impl CfgBuilder {
  /// RAII enter. C++ ctor saves `builder.currentBlock` and sets it to `target`:
  ///   `: builder(builder), saved(builder.currentBlock.get())
  ///    { builder.currentBlock = NotNull{target}; }`
  /// Here we apply the builder mutation (`currentBlock = target`); the matching
  /// restore (the dtor's job) is performed by the lowering scope that holds the
  /// saved block (see `lower_ast_stat_if` / `lower_ast_stat_while`).
  pub fn block_scope_cfg_builder_block(&mut self, target: *mut Block) {
    self.current_block = target;
  }
}
