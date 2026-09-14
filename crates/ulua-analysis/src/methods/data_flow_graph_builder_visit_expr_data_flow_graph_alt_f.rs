use alloc::string::String;
use core::{
  ffi::{CStr, c_void},
  ptr::NonNull,
};

use ulua_ast::records::ast_expr_index_name::AstExprIndexName;

use crate::{
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult, def::Def,
  },
  type_aliases::{def_id_def::DefId, def_id_refinement},
};
fn refinement_def_id(def: DefId) -> def_id_refinement::DefId {
  NonNull::new(def as *mut *const Def).unwrap()
}

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_index_name(
    &mut self,
    i: *mut AstExprIndexName,
  ) -> DataFlowResult {
    unsafe {
      let parent = self.visit_expr_ast_expr((*i).expr);
      let index = String::from(CStr::from_ptr((*i).index.value).to_string_lossy());
      let def = self.lookup_def_id_string_location(
        parent.def as *const Def,
        &index,
        (*i).base.base.location,
      );
      let key = (*self.key_arena).node(parent.parent, refinement_def_id(def), &index);

      DataFlowResult {
        def: def as *const c_void,
        parent: key,
      }
    }
  }
}
