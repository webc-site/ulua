//! Source: `Analysis/src/ControlFlowGraph.cpp:135-143` (hand-ported)
//! C++ `std::unique_ptr<ControlFlowGraph> CFGBuilder::makeCFG(NotNull<CFGAllocator> allocator, AstStatBlock* block)`.
use alloc::boxed::Box;

use ulua_ast::records::ast_stat_block::AstStatBlock;
use ulua_common::FFlag;

use crate::records::{
  cfg_allocator::CfgAllocator, cfg_builder::CfgBuilder, control_flow_graph::ControlFlowGraph,
};
impl CfgBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn make_cfg(
    allocator: *mut CfgAllocator,
    block: *mut AstStatBlock,
  ) -> *mut ControlFlowGraph {
    // C++:
    //   CFGBuilder builder(allocator);
    //   builder.lower(block);
    let mut builder = CfgBuilder::new(allocator);
    // `block` is `AstStatBlock*`; C++ `lower(block)` dispatches to the
    // `AstStatBlock*` overload.
    unsafe { builder.lower_ast_stat_block(block) };

    // auto cfg = std::move(builder.cfg);
    let cfg = builder.cfg.take().unwrap();

    // if (FFlag::DebugLuauFreezeArena) allocator->freeze();
    if FFlag::DebugLuauFreezeArena.get() {
      unsafe { (*allocator).freeze() };
    }

    // return cfg;  (unique_ptr -> raw owning pointer)
    Box::into_raw(Box::new(cfg))
  }
}
