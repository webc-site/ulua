//! Source: `Analysis/src/ConstraintGenerator.cpp:3540-3548` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprInterpString* interpString)`.
use core::ptr::null_mut;

use ulua_ast::records::ast_expr_interp_string::AstExprInterpString;

use crate::{
  enums::type_context::TypeContext,
  records::{
    constraint_generator::ConstraintGenerator, in_conditional_context::InConditionalContext,
    inference::Inference,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_interp_string(
    &mut self,
    scope: &ScopePtr,
    interp_string: *mut AstExprInterpString,
  ) -> Inference {
    unsafe {
      let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

      let expressions = (*interp_string).expressions;
      for &expr in expressions.as_slice() {
        self.check_scope_ptr_ast_expr(scope, expr);
      }

      Inference::inference_type_id_refinement_id((*self.builtin_types).string_type, null_mut())
    }
  }
}
