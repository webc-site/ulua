//! Source: `Analysis/src/ControlFlowGraph.cpp:107-110` (hand-ported)
//! C++ `DefId CFGAllocator::newDefinition(Symbol sym, size_t version)`.
use crate::{
  records::{cfg_allocator::CfgAllocator, sym_def::SymDef, symbol::Symbol},
  type_aliases::def_id_control_flow_graph::DefId,
};

impl CfgAllocator {
  pub fn new_definition(&mut self, sym: Symbol, version: usize) -> DefId {
    // C++: return NotNull{defs.allocate(SymDef{sym, version})};
    self.defs.allocate(SymDef::new(sym, version))
  }
}
