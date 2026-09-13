//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Predicate.h:61:or_predicate`
//! Source: `Analysis/include/Luau/Predicate.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Predicate.h
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Location.h
//!   - includes -> source_file Analysis/include/Luau/LValue.h
//!   - includes -> source_file Common/include/Luau/Variant.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Predicate.h
//!   - type_ref <- type_alias Predicate (Analysis/include/Luau/Predicate.h)
//!   - type_ref <- method OrPredicate::OrPredicate (Analysis/include/Luau/Predicate.h)
//!   - type_ref <- record TypeChecker (Analysis/include/Luau/TypeInfer.h)
//!   - type_ref <- method TypeChecker::checkExpr (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method TypeChecker::resolve (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method TypeChecker::resolve (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method TypeChecker::resolve (Analysis/src/TypeInfer.cpp)
//! - outgoing:
//!   - type_ref -> method OrPredicate::OrPredicate (Analysis/include/Luau/Predicate.h)
//!   - type_ref -> type_alias PredicateVec (Analysis/include/Luau/Predicate.h)
//!   - translates_to -> rust_item OrPredicate

use crate::type_aliases::predicate_vec::PredicateVec;

#[derive(Debug, Clone)]
pub struct OrPredicate {
  pub lhs: PredicateVec,
  pub rhs: PredicateVec,
}
