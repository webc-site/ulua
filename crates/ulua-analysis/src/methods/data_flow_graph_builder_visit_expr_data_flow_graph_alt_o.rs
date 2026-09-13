use ulua_ast::records::ast_expr_instantiate::AstExprInstantiate;
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::records::{
  data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult,
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_instantiate(
    &mut self,
    i: *mut AstExprInstantiate,
  ) -> DataFlowResult {
    unsafe {
      if FFlag::LuauExplicitTypeInstantiationSupport.get() {
        for type_or_pack in (*i).type_arguments.iter() {
          if !type_or_pack.r#type.is_null() {
            self.visit_type_ast_type(type_or_pack.r#type);
          } else {
            LUAU_ASSERT!(!type_or_pack.type_pack.is_null());
            self.visit_type_pack_ast_type_pack(type_or_pack.type_pack);
          }
        }
      }

      self.visit_expr_ast_expr((*i).expr)
    }
  }
}
