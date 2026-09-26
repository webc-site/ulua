use crate::{
  records::{data_flow_graph::DataFlowGraph, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};

impl DataFlowGraph {
  pub fn get_symbol_from_def(&self, def: DefId) -> Option<Symbol> {
    // C++: if (auto ref = defToSymbol.find(def)) return *ref; return nullopt;
    self.def_to_symbol.find(&def).cloned()
  }
}
