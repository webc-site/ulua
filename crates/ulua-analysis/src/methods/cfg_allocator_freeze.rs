//! Source: `Analysis/src/ControlFlowGraph.cpp:112-118` (hand-ported)
//! C++ `void CFGAllocator::freeze()`.
use crate::records::cfg_allocator::CfgAllocator;

impl CfgAllocator {
  pub fn freeze(&mut self) {
    // C++:
    //   block.freeze();
    //   defs.freeze();
    //   refinementArena.freeze();
    //   frozen = true;
    self.block.freeze();
    self.defs.freeze();
    // `RefinementArena::freeze()` == `allocator.freeze()` over its
    // `TypedAllocator<Refinement>` (field is `pub(crate)`, same crate).
    self.refinement_arena.allocator.freeze();
    self.frozen = true;
  }
}
