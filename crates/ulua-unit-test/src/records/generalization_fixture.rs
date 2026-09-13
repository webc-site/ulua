//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/Generalization.test.cpp:22:generalization_fixture`
//! Source: `tests/Generalization.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Generalization.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Generalization.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Generalization.test.cpp
//!   - type_ref <- method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
//!   - type_ref <- method GeneralizationFixture::to_string (tests/Generalization.test.cpp)
//!   - type_ref <- method GeneralizationFixture::to_string (tests/Generalization.test.cpp)
//!   - type_ref <- method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//! - outgoing:
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record BuiltinTypes (Analysis/include/Luau/Type.h)
//!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
//!   - type_ref -> record DenseHashSet (Common/include/Luau/DenseHash.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - translates_to -> rust_item GeneralizationFixture

use alloc::{boxed::Box, sync::Arc};
use core::ptr::null;

use ulua_analysis::{
  records::{
    builtin_types::BuiltinTypes, scope::Scope, to_string_options::ToStringOptions,
    type_arena::TypeArena,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
use ulua_common::{FFlag, records::dense_hash_set::DenseHashSet};

use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
#[derive(Debug)]
pub struct GeneralizationFixture {
  pub arena: Box<TypeArena>,
  pub builtin_types: Box<BuiltinTypes>,
  pub global_scope: ScopePtr,
  pub scope: ScopePtr,
  pub opts: ToStringOptions,
  pub generalized_types: DenseHashSet<TypeId>,
  pub _sff: ScopedFastFlag,
}

impl GeneralizationFixture {
  pub fn new() -> Self {
    let arena = Box::new(TypeArena::default());
    let builtin_types = Box::new(BuiltinTypes::new());
    let global_scope = Arc::new(Scope::scope_type_pack_id(builtin_types.any_type_pack));
    let scope = Arc::new(Scope::new(&global_scope, 0));

    Self {
      arena,
      builtin_types,
      global_scope,
      scope,
      opts: ToStringOptions::default(),
      generalized_types: DenseHashSet::new(null()),
      _sff: ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
    }
  }
}

impl Default for GeneralizationFixture {
  fn default() -> Self {
    Self::new()
  }
}
