use core::{ffi::c_void, ptr::null};

use ulua_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinaryOp};

use crate::{
  functions::contains_subscripted_definition::contains_subscripted_definition,
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult, def::Def,
    symbol::Symbol,
  },
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `b` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_binary(&mut self, b: *mut AstExprBinary) -> DataFlowResult {
    unsafe {
      let left = self.visit_expr_ast_expr((*b).left);
      let right = self.visit_expr_ast_expr((*b).right);

      let subscripted = ((*b).op == AstExprBinaryOp::And || (*b).op == AstExprBinaryOp::Or)
        && (contains_subscripted_definition(left.def as *const Def)
          || contains_subscripted_definition(right.def as *const Def));

      let def =
        (*self.def_arena).fresh_cell(Symbol::default(), (*b).base.base.location, subscripted);

      DataFlowResult {
        def: def as *const c_void,
        parent: null(),
      }
    }
  }
}
