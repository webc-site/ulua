//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/BuiltinTypeFunctions.h:9:builtin_type_functions`
//! Source: `Analysis/include/Luau/BuiltinTypeFunctions.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/BuiltinTypeFunctions.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/BuiltinTypeFunctions.h
//!   - type_ref <- method BuiltinTypeFunctions::BuiltinTypeFunctions (Analysis/include/Luau/BuiltinTypeFunctions.h)
//!   - type_ref <- method BuiltinTypeFunctions::operator= (Analysis/include/Luau/BuiltinTypeFunctions.h)
//!   - type_ref <- record BuiltinTypes (Analysis/include/Luau/Type.h)
//!   - type_ref <- method BuiltinTypeFunctions::BuiltinTypeFunctions (Analysis/src/BuiltinTypeFunctions.cpp)
//!   - type_ref <- method BuiltinTypes::BuiltinTypes (Analysis/src/Type.cpp)
//!   - type_ref <- method Fixture::getBuiltinTypeFunctions (tests/Fixture.cpp)
//!   - type_ref <- record Fixture (tests/Fixture.h)
//!   - type_ref <- record SubtypeFixture (tests/Subtyping.test.cpp)
//!   - type_ref <- method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
//!   - type_ref <- record TFFixture (tests/TypeFunction.test.cpp)
//!   - type_ref <- method BuiltinTypeFunctions::addToScope (Analysis/src/BuiltinTypeFunctions.cpp)
//! - outgoing:
//!   - type_ref -> method BuiltinTypeFunctions::BuiltinTypeFunctions (Analysis/include/Luau/BuiltinTypeFunctions.h)
//!   - type_ref -> record TypeFunction (Analysis/include/Luau/TypeFunction.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record Scope (Analysis/include/Luau/Scope.h)
//!   - translates_to -> rust_item BuiltinTypeFunctions

use crate::records::type_function::TypeFunction;

// 27 builtin type-function entries (BuiltinTypeFunctions.h:9-55). The ctor
// and addToScope are their own method nodes; C++ deletes copy/assign.
#[derive(Debug)]
pub struct BuiltinTypeFunctions {
  pub user_func: TypeFunction,

  pub not_func: TypeFunction,
  pub len_func: TypeFunction,
  pub unm_func: TypeFunction,

  pub add_func: TypeFunction,
  pub sub_func: TypeFunction,
  pub mul_func: TypeFunction,
  pub div_func: TypeFunction,
  pub idiv_func: TypeFunction,
  pub pow_func: TypeFunction,
  pub mod_func: TypeFunction,

  pub concat_func: TypeFunction,

  pub and_func: TypeFunction,
  pub or_func: TypeFunction,

  pub lt_func: TypeFunction,
  pub le_func: TypeFunction,

  pub refine_func: TypeFunction,
  pub singleton_func: TypeFunction,
  pub union_func: TypeFunction,
  pub intersect_func: TypeFunction,

  pub keyof_func: TypeFunction,
  pub rawkeyof_func: TypeFunction,
  pub index_func: TypeFunction,
  pub rawget_func: TypeFunction,

  pub setmetatable_func: TypeFunction,
  pub getmetatable_func: TypeFunction,

  pub objectof_func: TypeFunction,

  pub weakoptional_func: TypeFunction,
}
