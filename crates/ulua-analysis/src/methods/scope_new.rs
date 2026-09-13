//! C++ `Scope::Scope(const ScopePtr& parent, int sub_level = 0)`
//! (`Analysis/src/Scope.cpp`): a child scope inherits its parent's return type
//! and an incremented type level, and value-initializes every container. The
//! `DenseHash*` empty-key sentinels match the in-class initializers in
//! `Analysis/include/Luau/Scope.h` (`{nullptr}`, `{""}`, `{{}}`).
use alloc::{string::String, vec::Vec};
use core::ptr::null;
use std::collections::HashMap;

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{def::Def, scope::Scope},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl Scope {
  pub fn new(parent: &ScopePtr, sub_level: i32) -> Self {
    let mut level = parent.level.incr();
    level.sub_level = sub_level;

    Scope {
      parent: Some(parent.clone()),
      children: Vec::new(),
      bindings: HashMap::new(),
      return_type: parent.return_type,
      vararg_pack: None,
      level,
      location: Location::default(),
      exported_type_bindings: HashMap::new(),
      private_type_bindings: HashMap::new(),
      type_alias_locations: HashMap::new(),
      type_alias_name_locations: HashMap::new(),
      imported_modules: HashMap::new(),
      imported_type_bindings: HashMap::new(),
      builtin_type_names: DenseHashSet::default(),
      private_type_pack_bindings: HashMap::new(),
      refinements: HashMap::new(),
      lvalue_types: DenseHashMap::new(null::<Def>()),
      rvalue_refinements: DenseHashMap::new(null::<Def>()),
      globals_to_warn: DenseHashSet::default(),
      type_alias_type_parameters: HashMap::new(),
      type_alias_type_pack_parameters: HashMap::new(),
      interior_free_types: None,
      interior_free_type_packs: None,
      invalid_type_aliases: DenseHashMap::new(String::new()),
    }
  }
}
