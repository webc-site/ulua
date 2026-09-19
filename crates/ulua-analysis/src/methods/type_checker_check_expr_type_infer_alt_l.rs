use ulua_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;

use crate::{
  records::{
    type_checker::TypeChecker, types_are_unrelated::TypesAreUnrelated,
    with_predicate::WithPredicate,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn check_expr_scope_ptr_ast_expr_type_assertion(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprTypeAssertion,
  ) -> WithPredicate<TypeId> {
    let annotation_type = self.resolve_type(scope.clone(), unsafe { &*expr.annotation });
    let result = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
      scope,
      unsafe { &*expr.expr },
      Some(annotation_type),
      false,
    );

    if self
      .can_unify_type_id_type_id_scope_ptr_location(
        annotation_type,
        result.r#type,
        scope,
        &expr.base.base.location,
      )
      .is_empty()
    {
      return WithPredicate::with_predicate_t_predicate_vec(annotation_type, result.predicates);
    }

    if self
      .can_unify_type_id_type_id_scope_ptr_location(
        result.r#type,
        annotation_type,
        scope,
        &expr.base.base.location,
      )
      .is_empty()
    {
      return WithPredicate::with_predicate_t_predicate_vec(annotation_type, result.predicates);
    }

    self.report_error_location_type_error_data(
      &expr.base.base.location,
      TypesAreUnrelated {
        left: result.r#type,
        right: annotation_type,
      }
      .into(),
    );
    WithPredicate::with_predicate_t_predicate_vec(
      self.error_recovery_type_type_id(annotation_type),
      result.predicates,
    )
  }
}
