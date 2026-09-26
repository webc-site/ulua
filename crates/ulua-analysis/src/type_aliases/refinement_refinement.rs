//! Source: `Analysis/include/Luau/Refinement.h`

// Refinement.h:21 — using Refinement = Variant<Variadic, Negation, Conjunction,
//                                              Disjunction, Equivalence, Proposition>
use crate::{
  macros::variant_member,
  records::{
    conjunction_refinement::Conjunction, disjunction_refinement::Disjunction,
    equivalence::Equivalence, negation_refinement::Negation, proposition_refinement::Proposition,
    variadic::Variadic,
  },
};

#[derive(Debug, Clone)]
pub enum Refinement {
  Variadic(Variadic),
  Negation(Negation),
  Conjunction(Conjunction),
  Disjunction(Disjunction),
  Equivalence(Equivalence),
  Proposition(Proposition),
}

variant_member! {
  /// `get_if<T>(refinement.get())` over the refinement variant.
  enum RefinementMember: Refinement {
    Variadic => Variadic,
    Negation => Negation,
    Conjunction => Conjunction,
    Disjunction => Disjunction,
    Equivalence => Equivalence,
    Proposition => Proposition,
  }
}
