use alloc::sync::Arc;

use crate::{
  records::{scope::Scope, type_checker::TypeChecker},
  type_aliases::{predicate_vec::PredicateVec, scope_ptr_type::ScopePtr},
};
impl TypeChecker {
  pub fn resolve_predicate_vec_scope_ptr_bool(
    &mut self,
    predicates: &PredicateVec,
    scope: &ScopePtr,
    sense: bool,
  ) {
    let scope_mut = Arc::as_ptr(scope) as *mut Scope;
    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      predicates,
      unsafe { &mut (*scope_mut).refinements },
      scope,
      sense,
      false,
    );
  }
}
