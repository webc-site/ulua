//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Analysis:Analysis/src/TypeInfer.cpp:775:demoter`
//! Source: `Analysis/src/TypeInfer.cpp`
//! Graph edges:
//! - declared_by: source_file Analysis/src/TypeInfer.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/ApplyTypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/Cancellation.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Instantiation.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Quantify.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Substitution.h
//!   - includes -> source_file Common/include/Luau/TimeTrace.h
//!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypePack.h
//!   - includes -> source_file Analysis/include/Luau/TypeUtils.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//! - incoming:
//!   - declares <- source_file Analysis/src/TypeInfer.cpp
//!   - type_ref <- method TypeChecker::check (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method TypeChecker::getExpectedTypesForCall (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::Demoter (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::isDirty (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::isDirty (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::ignoreChildren (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::clean (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::clean (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::demotedLevel (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::demote (Analysis/src/TypeInfer.cpp)
//! - outgoing:
//!   - type_ref -> method Demoter::Demoter (Analysis/src/TypeInfer.cpp)
//!   - type_ref -> record Substitution (Analysis/include/Luau/Substitution.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record BuiltinTypes (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item Demoter

use crate::records::{builtin_types::BuiltinTypes, type_arena::TypeArena};
#[derive(Debug, Clone)]
pub struct Demoter {
  pub(crate) arena: *mut TypeArena,
  pub(crate) builtins: *mut BuiltinTypes,
}
