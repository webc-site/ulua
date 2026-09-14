use crate::{
  records::{
    and_predicate::AndPredicate, not_predicate::NotPredicate, or_predicate::OrPredicate,
    type_checker::TypeChecker,
  },
  type_aliases::{predicate::Predicate, refinement_map::RefinementMap, scope_ptr_type::ScopePtr},
};

impl TypeChecker {
  pub fn resolve_and_predicate_refinement_map_scope_ptr_bool(
    &mut self,
    and_p: &AndPredicate,
    refis: &mut RefinementMap,
    scope: &ScopePtr,
    sense: bool,
  ) {
    if !sense {
      let or_p = OrPredicate {
        lhs: alloc::vec![Predicate::Not(NotPredicate {
          predicates: and_p.lhs.clone(),
        })],
        rhs: alloc::vec![Predicate::Not(NotPredicate {
          predicates: and_p.rhs.clone(),
        })],
      };

      self.resolve_or_predicate_refinement_map_scope_ptr_bool(&or_p, refis, scope, !sense);
      return;
    }

    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      &and_p.lhs, refis, scope, sense, false,
    );
    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      &and_p.rhs, refis, scope, sense, false,
    );
  }
}
