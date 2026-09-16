use core::{ffi::c_void, ptr::NonNull};

use ulua_ast::records::ast_expr_global::AstExprGlobal;

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
  /// 调用方须保证 `g` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_global(&mut self, g: *mut AstExprGlobal) -> DataFlowResult {
    unsafe {
      let name = (*g).name;
      let def = self.lookup_symbol_location(Symbol::from_global(name), (*g).base.base.location);
      *self.graph.def_to_symbol.get_or_insert(def) = Symbol::from_global(name);
      let key = (*self.key_arena).leaf(refinement_def_id(def));

      DataFlowResult {
        def: def as *const c_void,
        parent: key,
      }
    }
  }
}
