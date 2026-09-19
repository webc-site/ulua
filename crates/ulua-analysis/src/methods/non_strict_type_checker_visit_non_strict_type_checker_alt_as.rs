use ulua_ast::records::ast_expr_instantiate::AstExprInstantiate;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr_instantiate(
    &mut self,
    instantiate: *mut AstExprInstantiate,
  ) -> NonStrictContext {
    unsafe {
      let type_arguments = (*instantiate).type_arguments;
      for param in type_arguments.as_slice() {
        if !param.r#type.is_null() {
          self.visit_ast_type(param.r#type);
        } else {
          self.visit_ast_type_pack(param.type_pack);
        }
      }

      self.visit_ast_expr_value_context((*instantiate).expr, ValueContext::RValue)
    }
  }
}
