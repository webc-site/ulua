use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `local` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_local(&mut self, local: *mut AstStatLocal) -> NonStrictContext {
    let local_ref = unsafe { &*local };
    let values = local_ref.values;
    for &rhs in values.as_slice() {
      self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
    }
    NonStrictContext::new()
  }
}
