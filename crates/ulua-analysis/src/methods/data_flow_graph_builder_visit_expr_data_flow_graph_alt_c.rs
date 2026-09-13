use core::{ffi::c_void, ptr::NonNull};

use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::{
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult, def::Def,
    symbol::Symbol,
  },
  type_aliases::{def_id_def::DefId, def_id_refinement},
};
fn refinement_def_id(def: DefId) -> def_id_refinement::DefId {
  NonNull::new(def as *mut *const Def).unwrap()
}

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_local(&mut self, l: *mut AstExprLocal) -> DataFlowResult {
    unsafe {
      let local = (*l).local;
      let def = self.lookup_symbol_location(Symbol::from_local(local), (*local).location);
      let key = (*self.key_arena).leaf(refinement_def_id(def));
      *self.graph.def_to_symbol.get_or_insert(def) = Symbol::from_local(local);

      DataFlowResult {
        def: def as *const c_void,
        parent: key,
      }
    }
  }
}
