//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/Simplify.test.cpp:17:simplify_fixture`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//!   - type_ref <- method SimplifyFixture::SimplifyFixture (tests/Simplify.test.cpp)
//!   - type_ref <- method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - type_ref <- method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
//!   - type_ref <- method SimplifyFixture::isIntersection (tests/Simplify.test.cpp)
//!   - type_ref <- method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
//!   - type_ref <- method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
//!   - type_ref <- method SimplifyFixture::mkFunction (tests/Simplify.test.cpp)
//!   - type_ref <- method SimplifyFixture::union_ (tests/Simplify.test.cpp)
//! - outgoing:
//!   - type_ref -> method SimplifyFixture::SimplifyFixture (tests/Simplify.test.cpp)
//!   - type_ref -> record Fixture (tests/Fixture.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record PendingExpansionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record SingletonType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record StringSingleton (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - translates_to -> rust_item SimplifyFixture

use ulua_analysis::{
  records::{to_string_options::ToStringOptions, type_arena::TypeArena},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId},
};

use crate::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

#[derive(Debug)]
pub struct SimplifyFixture {
  pub sff_debug_luau_force_old_solver: ScopedFastFlag,
  pub any_ty: TypeId,
  pub unknown_ty: TypeId,
  pub never_ty: TypeId,
  pub error_ty: TypeId,
  pub function_ty: TypeId,
  pub table_ty: TypeId,
  pub number_ty: TypeId,
  pub string_ty: TypeId,
  pub boolean_ty: TypeId,
  pub nil_ty: TypeId,
  pub class_ty: TypeId,
  pub true_ty: TypeId,
  pub false_ty: TypeId,
  pub truthy_ty: TypeId,
  pub falsy_ty: TypeId,

  pub free_ty: TypeId,
  pub generic_ty: TypeId,
  pub blocked_ty: TypeId,
  pub pending_ty: TypeId,
  pub hello_ty: TypeId,
  pub world_ty: TypeId,

  pub empty_type_pack: TypePackId,
  pub fn1_ty: TypeId,
  pub fn2_ty: TypeId,

  pub parent_class_ty: TypeId,
  pub child_class_ty: TypeId,
  pub another_child_class_ty: TypeId,
  pub unrelated_class_ty: TypeId,
  pub scope: ScopePtr,
  pub opts: ToStringOptions,
  pub arena: TypeArena,
  pub base: Fixture,
}

impl Default for SimplifyFixture {
  fn default() -> Self {
    Self::new()
  }
}
