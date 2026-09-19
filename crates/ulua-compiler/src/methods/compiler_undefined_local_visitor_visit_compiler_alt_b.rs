use ulua_ast::records::ast_expr_function::AstExprFunction;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::undefined_local_visitor::UndefinedLocalVisitor;

impl UndefinedLocalVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
    unsafe {
      let f = (*self.self_).functions.find(&node);
      LUAU_ASSERT!(f.is_some());
      let f = f.unwrap();

      for uv in &f.upvals {
        LUAU_ASSERT!((*(*uv)).function_depth < (*node).function_depth);

        if (*(*uv)).function_depth == (*node).function_depth - 1 {
          self.check(*uv);
        }
      }
    }
    false
  }
}
