//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Type.h:1118:union_type_iterator`
//! Source: `Analysis/include/Luau/Type.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Type.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Polarity.h
//!   - includes -> source_file Analysis/include/Luau/Predicate.h
//!   - includes -> source_file Analysis/include/Luau/Refinement.h
//!   - includes -> source_file Analysis/include/Luau/Unifiable.h
//!   - includes -> source_file Common/include/Luau/Variant.h
//!   - includes -> source_file Common/include/Luau/VecDeque.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Type.h
//!   - type_ref <- record TypeIterator (Analysis/include/Luau/Type.h)
//!   - type_ref <- method Normalizer::unionNormalWithTy (Analysis/src/Normalize.cpp)
//!   - type_ref <- method Normalizer::intersectNormalWithTy (Analysis/src/Normalize.cpp)
//!   - type_ref <- function begin (Analysis/src/Type.cpp)
//!   - type_ref <- function end (Analysis/src/Type.cpp)
//! - outgoing:
//!   - type_ref -> record TypeIterator (Analysis/include/Luau/Type.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item UnionTypeIterator

use crate::records::{type_iterator::TypeIterator, union_type::UnionType};

pub type UnionTypeIterator = TypeIterator<UnionType>;
