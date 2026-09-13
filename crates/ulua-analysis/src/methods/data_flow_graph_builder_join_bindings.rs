use alloc::vec::Vec;

use crate::{
  records::{data_flow_graph_builder::DataFlowGraphBuilder, dfg_scope::DfgScope, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `p` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// `void DataFlowGraphBuilder::joinBindings(...)`.
  ///
  /// Same borrow-safety concern as [`join_props`](DataFlowGraphBuilder::join_props):
  /// the `while`-loop visitor calls `join(scope, scope, whileScope)`, so `p` aliases
  /// `a`. Iterating `a.bindings` while `get_or_insert` mutates `p.bindings` (the same
  /// map) is `&`/`&mut` aliasing UB. Snapshot `a`/`b` bindings into owned vectors first
  /// so no borrow of a scope's `bindings` is held across the mutation of `p.bindings`.
  pub unsafe fn join_bindings(&mut self, p: *mut DfgScope, a: &DfgScope, b: &DfgScope) {
    unsafe {
      let a_bindings: Vec<(Symbol, DefId)> =
        a.bindings.iter().map(|(s, d)| (s.clone(), *d)).collect();
      let b_bindings: Vec<(Symbol, DefId)> =
        b.bindings.iter().map(|(s, d)| (s.clone(), *d)).collect();
      let b_find = |sym: &Symbol| -> Option<DefId> {
        b_bindings.iter().find(|(s, _)| s == sym).map(|(_, d)| *d)
      };

      for (sym, def1) in a_bindings.iter() {
        if let Some(def2) = b_find(sym) {
          let phi = (*self.def_arena).phi_def_id_def_id(*def1, def2);
          *(*p).bindings.get_or_insert(sym.clone()) = phi;
        } else if let Some(def2) = (*p).lookup_symbol(sym.clone()) {
          let phi = (*self.def_arena).phi_def_id_def_id(*def1, def2);
          *(*p).bindings.get_or_insert(sym.clone()) = phi;
        }
      }

      for (sym, def1) in b_bindings.iter() {
        if let Some(def2) = (*p).lookup_symbol(sym.clone()) {
          let phi = (*self.def_arena).phi_def_id_def_id(*def1, def2);
          *(*p).bindings.get_or_insert(sym.clone()) = phi;
        }
      }
    }
  }
}
