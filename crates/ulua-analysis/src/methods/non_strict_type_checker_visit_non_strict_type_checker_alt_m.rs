use ulua_ast::records::ast_stat_assign::AstStatAssign;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `assign` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_assign(&mut self, assign: *mut AstStatAssign) -> NonStrictContext {
    unsafe {
      let assign_ref = &*assign;
      let vars = assign_ref.vars;
      let values = assign_ref.values;

      for &lhs in vars.as_slice() {
        self.visit_ast_expr_value_context(lhs, ValueContext::LValue);
      }

      for &rhs in values.as_slice() {
        self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
      }
    }

    NonStrictContext::new()
  }
}
