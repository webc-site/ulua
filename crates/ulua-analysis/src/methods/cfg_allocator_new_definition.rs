//! Source: `Analysis/src/ControlFlowGraph.cpp:107-110` (hand-ported)
//! C++ `DefId CFGAllocator::newDefinition(Symbol sym, size_t version)`.
use crate::{
  records::{
    cfg_allocator::CfgAllocator, sym_def::SymDef, sym_def_registry::register_sym_def,
    symbol::Symbol,
  },
  type_aliases::def_id_control_flow_graph::DefId,
};

impl CfgAllocator {
  pub fn new_definition(&mut self, sym: Symbol, version: usize) -> DefId {
    // C++: return NotNull{defs.allocate(SymDef{sym, version})};
    // 裸指针仅在注册点出现：arena 地址经 `register_sym_def` 换成 u32 句柄，
    // 之后全链（指令操作数、reaching definitions、use-def、refinement 命题）
    // 只持有/比较句柄（见 `records::sym_def_registry` 模块契约）。
    register_sym_def(self.defs.allocate(SymDef::new(sym, version)))
  }
}
