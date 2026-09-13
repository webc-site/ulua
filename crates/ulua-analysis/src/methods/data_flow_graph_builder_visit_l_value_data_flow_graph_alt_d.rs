use alloc::string::String;
use core::ffi::CStr;

use ulua_ast::records::ast_expr_index_name::AstExprIndexName;

use crate::{
  functions::contains_subscripted_definition::contains_subscripted_definition,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_l_value_ast_expr_index_name_def_id(
    &mut self,
    i: *mut AstExprIndexName,
    incoming_def: DefId,
  ) -> DefId {
    // C++:
    //   DefId parentDef = visitExpr(i->expr).def;
    //   DfgScope* scope = currentScope();
    //   DefId updated = defArena->freshCell(i->index, i->location, containsSubscriptedDefinition(incomingDef));
    //   scope->props[parentDef][i->index.value] = updated;
    //   return updated;
    unsafe {
      let parent_def = self.visit_expr_ast_expr((*i).expr).def as DefId;
      let scope = self.current_scope();
      let index_str = String::from(CStr::from_ptr((*i).index.value).to_string_lossy());
      let subscripted = contains_subscripted_definition(incoming_def);
      let updated = (*self.def_arena).fresh_cell(
        Symbol::from_global((*i).index),
        (*i).base.base.location,
        subscripted,
      );
      (*scope)
        .props
        .get_or_insert(parent_def)
        .insert(index_str, updated);
      updated
    }
  }
}
