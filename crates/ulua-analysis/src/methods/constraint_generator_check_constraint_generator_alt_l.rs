use core::ptr::null_mut;

use ulua_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;

use crate::{
  enums::polarity::Polarity,
  records::{constraint_generator::ConstraintGenerator, inference::Inference},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_type_assertion(
    &mut self,
    scope: &ScopePtr,
    type_assert: *mut AstExprTypeAssertion,
  ) -> Inference {
    let type_assert_ref = unsafe { &*type_assert };
    self.check_scope_ptr_ast_expr(scope, type_assert_ref.expr);
    Inference::inference_type_id_refinement_id(
      self.resolve_type(
        scope.as_ref() as *const _ as *mut _,
        type_assert_ref.annotation,
        false,
        false,
        Polarity::Positive,
      ),
      null_mut(),
    )
  }
}
