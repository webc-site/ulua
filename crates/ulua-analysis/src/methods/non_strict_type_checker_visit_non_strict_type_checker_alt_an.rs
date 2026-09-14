use ulua_ast::records::ast_expr_binary::AstExprBinary;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};
impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `binary` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_binary(&mut self, binary: *mut AstExprBinary) -> NonStrictContext {
    unsafe {
      let n = &*binary;
      let lhs = self.visit_ast_expr_value_context(n.left, ValueContext::RValue);
      let rhs = self.visit_ast_expr_value_context(n.right, ValueContext::RValue);
      let builtin_types = self.builtin_types;
      let arena = self.arena;
      NonStrictContext::disjunction(builtin_types, arena, &lhs, &rhs)
    }
  }
}
