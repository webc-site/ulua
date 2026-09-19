use ulua_ast::records::ast_expr_if_else::AstExprIfElse;

use crate::{
  functions::reduce_union::reduce_union,
  records::{type_checker::TypeChecker, union_type::UnionType, with_predicate::WithPredicate},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn check_expr_scope_ptr_ast_expr_if_else_optional_type_id(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprIfElse,
    expected_type: Option<TypeId>,
  ) -> WithPredicate<TypeId> {
    let result = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
      scope,
      unsafe { &*expr.condition },
      None,
      false,
    );

    let true_scope = self.child_scope(scope, &unsafe { (*expr.true_expr).base.location });
    self.resolve_predicate_vec_scope_ptr_bool(&result.predicates, &true_scope, true);
    let true_type = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
      &true_scope,
      unsafe { &*expr.true_expr },
      expected_type,
      false,
    );

    let false_scope = self.child_scope(scope, &unsafe { (*expr.false_expr).base.location });
    self.resolve_predicate_vec_scope_ptr_bool(&result.predicates, &false_scope, false);
    let false_type = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
      &false_scope,
      unsafe { &*expr.false_expr },
      expected_type,
      false,
    );

    if false_type.r#type == true_type.r#type {
      return WithPredicate::with_predicate_t(true_type.r#type);
    }

    let types = reduce_union(&[true_type.r#type, false_type.r#type]);
    if types.is_empty() {
      return WithPredicate::with_predicate_t(self.never_type);
    }

    WithPredicate::with_predicate_t(if types.len() == 1 {
      types[0]
    } else {
      self.add_type(&UnionType { options: types })
    })
  }
}
