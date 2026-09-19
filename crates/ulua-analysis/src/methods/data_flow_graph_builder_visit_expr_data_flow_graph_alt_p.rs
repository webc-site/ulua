use core::{ffi::c_void, ptr::null};

use ulua_ast::records::ast_expr_error::AstExprError;

use crate::{
  enums::scope_type::ScopeType,
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult,
    dfg_scope::DfgScope, push_scope::PushScope, symbol::Symbol,
  },
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `error` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_error(&mut self, error: *mut AstExprError) -> DataFlowResult {
    unsafe {
      {
        let unreachable: *mut DfgScope = self.make_child_scope(ScopeType::Linear);
        let _ps = PushScope::new(&mut self.scope_stack, unreachable);

        for expr in (*error).expressions.iter() {
          self.visit_expr_ast_expr(*expr);
        }
      }

      let def = (*self.def_arena).fresh_cell(Symbol::default(), (*error).base.base.location, false);
      DataFlowResult {
        def: def as *const c_void,
        parent: null(),
      }
    }
  }
}
