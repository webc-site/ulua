// ConstraintGenerator::visit(const ScopePtr&, AstStatIf*) (ConstraintGenerator.cpp:2055-2090).
use ulua_ast::records::{ast_node::AstNode, ast_stat::AstStat, ast_stat_if::AstStatIf};

use crate::{
  enums::{control_flow::ControlFlow, type_context::TypeContext},
  functions::matches::matches,
  records::{
    constraint_generator::ConstraintGenerator, in_conditional_context::InConditionalContext,
    scope::Scope,
  },
  type_aliases::{refinement_id_refinement::RefinementId, scope_ptr_type::ScopePtr},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_if(
    &mut self,
    scope: &ScopePtr,
    if_statement: *mut AstStatIf,
  ) -> ControlFlow {
    let if_ref = unsafe { &*if_statement };

    let refinement: RefinementId = {
      let _flipper =
        unsafe { InConditionalContext::new(&mut self.type_context, TypeContext::Condition) };
      self
        .check_scope_ptr_ast_expr_optional_type_id(scope, if_ref.condition, None)
        .refinement
    };

    let then_scope: ScopePtr = unsafe { self.child_scope(if_ref.thenbody as *mut AstNode, scope) };
    self.apply_refinements(
      &then_scope,
      unsafe { (*if_ref.condition).base.location },
      refinement,
    );

    let else_node: *mut AstNode = if !if_ref.elsebody.is_null() {
      if_ref.elsebody as *mut AstNode
    } else {
      &if_ref.base.base as *const AstNode as *mut AstNode
    };
    let else_scope: ScopePtr = unsafe { self.child_scope(else_node, scope) };
    let else_refinement_location = if_ref
      .else_location
      .unwrap_or(unsafe { (*if_ref.condition).base.location });
    let negated = self.refinement_arena.negation(refinement);
    self.apply_refinements(&else_scope, else_refinement_location, negated);

    let thencf =
      unsafe { self.visit_scope_ptr_ast_stat(&then_scope, if_ref.thenbody as *mut AstStat) };
    let mut elsecf = ControlFlow::None;
    if !if_ref.elsebody.is_null() {
      elsecf = unsafe { self.visit_scope_ptr_ast_stat(&else_scope, if_ref.elsebody) };
    }

    if thencf != ControlFlow::None && elsecf == ControlFlow::None {
      unsafe { (*(scope.as_ref() as *const Scope as *mut Scope)).inherit_refinements(&else_scope) };
    } else if thencf == ControlFlow::None && elsecf != ControlFlow::None {
      unsafe { (*(scope.as_ref() as *const Scope as *mut Scope)).inherit_refinements(&then_scope) };
    } else if thencf == ControlFlow::None && elsecf == ControlFlow::None {
      self.inherit_shared_refinements(
        scope,
        unsafe { (*if_ref.condition).base.location },
        &then_scope,
        &else_scope,
      );
    }

    if thencf == ControlFlow::None {
      unsafe { (*(scope.as_ref() as *const Scope as *mut Scope)).inherit_assignments(&then_scope) };
    }
    if elsecf == ControlFlow::None {
      unsafe { (*(scope.as_ref() as *const Scope as *mut Scope)).inherit_assignments(&else_scope) };
    }

    if thencf == elsecf {
      thencf
    } else if (matches(thencf, ControlFlow::Returns) || matches(thencf, ControlFlow::Throws))
      && (matches(elsecf, ControlFlow::Returns) || matches(elsecf, ControlFlow::Throws))
    {
      ControlFlow::Returns
    } else {
      ControlFlow::None
    }
  }
}
