//! Source: `tests/Generalization.test.cpp`

use alloc::{boxed::Box, sync::Arc};
use core::ptr::null;

use ulua_analysis::{
  records::{
    builtin_types::BuiltinTypes, scope::Scope, to_string_options::ToStringOptions,
    type_arena::TypeArena,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
use ulua_common::{fflag, records::dense_hash_set::DenseHashSet};

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
      _sff: ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    }
  }
}

impl Default for GeneralizationFixture {
  fn default() -> Self {
    Self::new()
  }
}
