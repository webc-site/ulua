use ulua_ast::records::ast_expr::AstExpr;

use crate::{records::type_checker_2::TypeChecker2, type_aliases::type_id::TypeId};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn test_literal_or_ast_type_is_subtype(
    &mut self,
    expr: *mut AstExpr,
    expected_type: TypeId,
  ) -> bool {
    let scope = self.find_innermost_scope(unsafe { (*expr).base.location });
    // SAFETY: expr 指向 AST arena 节点。
    let expr_ty = self.lookup_type(unsafe { &*expr });

    let r = unsafe {
      (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(expr_ty, expected_type, scope)
    };

    if r.is_subtype {
      return true;
    }

    // SAFETY: expr 指向 AST arena 节点。
    self.test_potential_literal_is_subtype(unsafe { &*expr }, expected_type)
  }
}
