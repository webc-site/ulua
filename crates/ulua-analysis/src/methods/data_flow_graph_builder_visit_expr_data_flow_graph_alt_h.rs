use ulua_ast::records::ast_expr_function::AstExprFunction;

use crate::{
  enums::scope_type::ScopeType,
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult,
    push_scope::PushScope,
  },
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `f` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_function(&mut self, f: *mut AstExprFunction) -> DataFlowResult {
    let signature_scope = self.make_child_scope(ScopeType::Function);
    let _ps = PushScope::new(&mut self.scope_stack, signature_scope);

    unsafe { self.visit_function(f, signature_scope) }
  }
}
