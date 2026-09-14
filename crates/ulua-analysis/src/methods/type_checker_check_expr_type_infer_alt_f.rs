use core::ffi::CStr;

use ulua_ast::records::ast_expr_index_name::AstExprIndexName;

use crate::{
  functions::try_get_l_value::try_get_l_value,
  records::{
    truthy_predicate::TruthyPredicate, type_checker::TypeChecker, with_predicate::WithPredicate,
  },
  type_aliases::{
    name_type::Name, predicate::Predicate, predicate_vec::PredicateVec, scope_ptr_type::ScopePtr,
    type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn check_expr_scope_ptr_ast_expr_index_name(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprIndexName,
  ) -> WithPredicate<TypeId> {
    let name: Name = unsafe {
      CStr::from_ptr(expr.index.value)
        .to_string_lossy()
        .into_owned()
    };

    // Redundant call if we find a refined lvalue, but this function must be called in order to recursively populate ast_types.
    let mut lhs_type = self
      .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        unsafe { &*expr.expr },
        None,
        false,
      )
      .r#type;

    if let Some(lvalue) = try_get_l_value(&expr.base)
      && let Some(ty) = self.resolve_l_value_scope_ptr_l_value(scope.clone(), &lvalue)
    {
      let predicate = TruthyPredicate {
        lvalue,
        location: expr.base.base.location,
      };
      return WithPredicate::with_predicate_t_predicate_vec(
        ty,
        PredicateVec::from(vec![Predicate::Truthy(predicate)]),
      );
    }

    lhs_type = self.strip_from_nil_and_report(lhs_type, unsafe { &(*expr.expr).base.location });

    if let Some(ty) = self.get_index_type_from_type(
      scope.clone(),
      lhs_type,
      &name,
      &expr.base.base.location,
      true,
    ) {
      return WithPredicate::with_predicate_t(ty);
    }

    WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
  }
}
