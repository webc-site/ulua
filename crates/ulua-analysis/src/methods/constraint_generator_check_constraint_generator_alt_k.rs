//! Source: `Analysis/src/ConstraintGenerator.cpp:3512-3532` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprIfElse* ifElse, std::optional<TypeId> expected_type)`.
use core::ptr::null_mut;

use ulua_ast::records::{ast_expr_if_else::AstExprIfElse, ast_node::AstNode};

use crate::{
  enums::type_context::TypeContext,
  records::{
    constraint_generator::ConstraintGenerator, in_conditional_context::InConditionalContext,
    inference::Inference, scope::Scope,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_if_else_optional_type_id(
    &mut self,
    scope: &ScopePtr,
    if_else: *mut AstExprIfElse,
    expected_type: Option<TypeId>,
  ) -> Inference {
    unsafe {
      let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

      let refinement = {
        // C++ `InConditionalContext flipper{&typeContext}` -> default newValue is Condition.
        let _flipper = InConditionalContext::new(&mut self.type_context, TypeContext::Condition);
        let cond_scope = self.child_scope((*if_else).condition as *mut AstNode, scope);
        self
          .check_scope_ptr_ast_expr(&cond_scope, (*if_else).condition)
          .refinement
      };

      let then_scope = self.child_scope((*if_else).true_expr as *mut AstNode, scope);
      self.apply_refinements(
        &then_scope,
        (*(*if_else).true_expr).base.location,
        refinement,
      );
      let then_type = self
        .check_scope_ptr_ast_expr_optional_type_id(&then_scope, (*if_else).true_expr, expected_type)
        .ty;

      let else_scope = self.child_scope((*if_else).false_expr as *mut AstNode, scope);
      let negated = self.refinement_arena.negation_refinement_id(refinement);
      self.apply_refinements(&else_scope, (*(*if_else).false_expr).base.location, negated);
      let else_type = self
        .check_scope_ptr_ast_expr_optional_type_id(
          &else_scope,
          (*if_else).false_expr,
          expected_type,
        )
        .ty;

      let union = self.make_union_scope_ptr_location_type_id_type_id(
        scope.as_ref() as *const Scope as *mut Scope,
        (*if_else).base.base.location,
        then_type,
        else_type,
      );
      Inference::inference_type_id_refinement_id(union, null_mut())
    }
  }
}
