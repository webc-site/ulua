use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{
    predicate_vec::PredicateVec, refinement_map::RefinementMap, scope_ptr_type::ScopePtr,
  },
};

impl TypeChecker {
  pub fn resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
    &mut self,
    predicates: &PredicateVec,
    refis: &mut RefinementMap,
    scope: &ScopePtr,
    sense: bool,
    from_or: bool,
  ) {
    for c in predicates {
      self.resolve_predicate_refinement_map_scope_ptr_bool_bool(c, refis, scope, sense, from_or);
    }
  }
}
