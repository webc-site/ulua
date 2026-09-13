use ulua_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinaryOp};

use crate::{
  functions::{
    try_get_l_value::try_get_l_value, try_get_type_guard_predicate::try_get_type_guard_predicate,
  },
  records::{
    and_predicate::AndPredicate, eq_predicate::EqPredicate, not_predicate::NotPredicate,
    or_predicate::OrPredicate, type_checker::TypeChecker, with_predicate::WithPredicate,
  },
  type_aliases::{
    predicate::Predicate, predicate_vec::PredicateVec, scope_ptr_type::ScopePtr, type_id::TypeId,
  },
};

impl TypeChecker {
  pub fn check_expr_scope_ptr_ast_expr_binary_optional_type_id(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprBinary,
    expected_type: Option<TypeId>,
  ) -> WithPredicate<TypeId> {
    if expr.op == AstExprBinaryOp::And {
      let lhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        unsafe { &*expr.left },
        expected_type,
        false,
      );
      let lhs_ty = lhs.r#type;
      let lhs_predicates = lhs.predicates;

      let inner_scope = self.child_scope(scope, &expr.base.base.location);
      self.resolve_predicate_vec_scope_ptr_bool(&lhs_predicates, &inner_scope, true);

      let rhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        &inner_scope,
        unsafe { &*expr.right },
        expected_type,
        false,
      );
      let rhs_ty = rhs.r#type;
      let rhs_predicates = rhs.predicates;

      let result_ty =
        self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &PredicateVec::new());
      WithPredicate::with_predicate_t_predicate_vec(
        result_ty,
        PredicateVec::from(alloc::vec![Predicate::And(AndPredicate {
          lhs: lhs_predicates,
          rhs: rhs_predicates,
        })]),
      )
    } else if expr.op == AstExprBinaryOp::Or {
      let lhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        unsafe { &*expr.left },
        expected_type,
        false,
      );
      let lhs_ty = lhs.r#type;
      let lhs_predicates = lhs.predicates;

      let inner_scope = self.child_scope(scope, &expr.base.base.location);
      self.resolve_predicate_vec_scope_ptr_bool(&lhs_predicates, &inner_scope, false);

      let rhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        &inner_scope,
        unsafe { &*expr.right },
        expected_type,
        false,
      );
      let rhs_ty = rhs.r#type;
      let rhs_predicates = rhs.predicates;

      // Because of C++, I'm not sure if lhsPredicates was not moved out by the time we call checkBinaryOperation.
      let result = self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &lhs_predicates);
      WithPredicate::with_predicate_t_predicate_vec(
        result,
        PredicateVec::from(alloc::vec![Predicate::Or(OrPredicate {
          lhs: lhs_predicates,
          rhs: rhs_predicates,
        })]),
      )
    } else if expr.op == AstExprBinaryOp::CompareEq || expr.op == AstExprBinaryOp::CompareNe {
      // For these, passing expected_type is worse than simply forcing them, because their implementation
      // may inadvertently check if expectedTypes exist first and use it, instead of forceSingleton first.
      let lhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        unsafe { &*expr.left },
        None,
        true,
      );
      let rhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        unsafe { &*expr.right },
        None,
        true,
      );
      let lhs_ty = lhs.r#type;
      let rhs_ty = rhs.r#type;

      if let Some(predicate) = try_get_type_guard_predicate(expr) {
        return WithPredicate::with_predicate_t_predicate_vec(
          self.boolean_type,
          PredicateVec::from(alloc::vec![predicate]),
        );
      }

      let mut predicates: PredicateVec = PredicateVec::new();

      if let Some(lvalue) = try_get_l_value(unsafe { &*expr.left }) {
        predicates.push(Predicate::Eq(EqPredicate {
          lvalue,
          ty: rhs_ty,
          location: expr.base.base.location,
        }));
      }

      if let Some(lvalue) = try_get_l_value(unsafe { &*expr.right }) {
        predicates.push(Predicate::Eq(EqPredicate {
          lvalue,
          ty: lhs_ty,
          location: expr.base.base.location,
        }));
      }

      if !predicates.is_empty() && expr.op == AstExprBinaryOp::CompareNe {
        predicates = PredicateVec::from(alloc::vec![Predicate::Not(NotPredicate { predicates })]);
      }

      let result_ty =
        self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &PredicateVec::new());
      WithPredicate::with_predicate_t_predicate_vec(result_ty, predicates)
    } else {
      // Expected type_arguments are not useful for other binary operators.
      let lhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        unsafe { &*expr.left },
        None,
        false,
      );
      let rhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        unsafe { &*expr.right },
        None,
        false,
      );
      let lhs_ty = lhs.r#type;
      let rhs_ty = rhs.r#type;
      let lhs_predicates = lhs.predicates;

      // Intentionally discarding predicates with other operators.
      let result_ty = self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &lhs_predicates);
      WithPredicate::with_predicate_t(result_ty)
    }
  }
}
