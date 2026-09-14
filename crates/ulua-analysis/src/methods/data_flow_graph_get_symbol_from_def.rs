use crate::records::{data_flow_graph::DataFlowGraph, def::Def, symbol::Symbol};

impl DataFlowGraph {
  pub fn get_symbol_from_def(&self, def: *const Def) -> Option<Symbol> {
    // C++: if (auto ref = defToSymbol.find(def)) return *ref; return nullopt;
    self.def_to_symbol.find(&def).cloned()
  }
}
