//! C++ `Scope::Scope(TypePackId returnType)` (`Analysis/src/Scope.cpp:10`):
//! a root scope with no parent, the given return type, and a default
//! `TypeLevel{}`. Every other member uses its in-class initializer (the
//! `DenseHash*` empty-key sentinels from `Analysis/include/Luau/Scope.h`).
use alloc::{string::String, vec::Vec};
use core::ptr::null;
use std::collections::HashMap;

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{def::Def, scope::Scope, type_level::TypeLevel},
  type_aliases::type_pack_id::TypePackId,
};
impl Scope {
  pub fn scope_type_pack_id(return_type: TypePackId) -> Self {
    Scope {
      parent: None,
      children: Vec::new(),
      bindings: HashMap::new(),
      return_type,
      vararg_pack: None,
      level: TypeLevel::default(),
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
