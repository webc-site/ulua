use core::ffi::CStr;

use ulua_ast::records::ast_expr_global::AstExprGlobal;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::try_get_l_value::try_get_l_value,
  records::{
    truthy_predicate::TruthyPredicate,
    type_checker::TypeChecker,
    type_error::TypeError,
    unknown_symbol::{Context, UnknownSymbol},
    with_predicate::WithPredicate,
  },
  type_aliases::{
    predicate::Predicate, predicate_vec::PredicateVec, scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn check_expr_scope_ptr_ast_expr_global(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprGlobal,
  ) -> WithPredicate<TypeId> {
    let lvalue = try_get_l_value(&expr.base);
    LUAU_ASSERT!(lvalue.is_some());

    if let Some(ty) =
      self.resolve_l_value_scope_ptr_l_value(scope.clone(), &lvalue.clone().unwrap())
    {
      let predicate = TruthyPredicate {
        lvalue: lvalue.unwrap(),
        location: expr.base.base.location,
      };
      return WithPredicate::with_predicate_t_predicate_vec(
        ty,
        PredicateVec::from(vec![Predicate::Truthy(predicate)]),
      );
    }

    let name_str = unsafe { CStr::from_ptr(expr.name.value).to_string_lossy() };
    let error_data =
      TypeErrorData::UnknownSymbol(UnknownSymbol::new(name_str.to_string(), Context::Binding));
    let error = TypeError::type_error_location_type_error_data(expr.base.base.location, error_data);
    self.report_error_type_error(&error);

    WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
  }
}
