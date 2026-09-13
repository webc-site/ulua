use crate::{
  records::{
    and_predicate::AndPredicate, not_predicate::NotPredicate, or_predicate::OrPredicate,
    type_checker::TypeChecker,
  },
  type_aliases::{predicate::Predicate, refinement_map::RefinementMap, scope_ptr_type::ScopePtr},
};

impl TypeChecker {
  pub fn resolve_or_predicate_refinement_map_scope_ptr_bool(
    &mut self,
    or_p: &OrPredicate,
    refis: &mut RefinementMap,
    scope: &ScopePtr,
    sense: bool,
  ) {
    if !sense {
      let and_p = AndPredicate {
        lhs: alloc::vec![Predicate::Not(NotPredicate {
          predicates: or_p.lhs.clone(),
        })],
        rhs: alloc::vec![Predicate::Not(NotPredicate {
          predicates: or_p.rhs.clone(),
        })],
      };

      self.resolve_and_predicate_refinement_map_scope_ptr_bool(&and_p, refis, scope, !sense);
      return;
    }

    let mut left_refis = RefinementMap::new();
    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      &or_p.lhs,
      &mut left_refis,
      scope,
      sense,
      false,
    );

    let mut right_refis = RefinementMap::new();
    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      &or_p.lhs,
      &mut right_refis,
      scope,
      !sense,
      false,
    );
    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      &or_p.rhs,
      &mut right_refis,
      scope,
      sense,
      true,
    );

    self.merge(refis, &left_refis);
    self.merge(refis, &right_refis);
  }
}
