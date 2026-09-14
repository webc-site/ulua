//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Scope.h:33:scope`
//! Source: `Analysis/include/Luau/Scope.h` (Scope.h:33-118, hand-ported; fields only,
//! methods are separate schedule items)

use alloc::{string::String, vec::Vec};
use std::collections::HashMap;

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{binding::Binding, def::Def, symbol::Symbol, type_fun::TypeFun, type_level::TypeLevel},
  type_aliases::{
    module_name_type::ModuleName, name_type::Name, refinement_map::RefinementMap,
    scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId,
  },
};

#[derive(Debug)]
pub struct Scope {
  pub parent: Option<ScopePtr>, // null for the root

  pub children: Vec<*mut Scope>, // NotNull<Scope>
  pub bindings: HashMap<Symbol, Binding>,
  pub return_type: TypePackId,
  pub vararg_pack: Option<TypePackId>,

  pub level: TypeLevel,

  /// the spanning location associated with this scope
  pub location: Location,

  pub exported_type_bindings: HashMap<Name, TypeFun>,
  pub private_type_bindings: HashMap<Name, TypeFun>,
  pub type_alias_locations: HashMap<Name, Location>,
  pub type_alias_name_locations: HashMap<Name, Location>,
  /// Mapping from the name in the require statement to the internal module_name.
  pub imported_modules: HashMap<Name, ModuleName>,
  pub imported_type_bindings: HashMap<Name, HashMap<Name, TypeFun>>,
  pub builtin_type_names: DenseHashSet<Name>,

  pub private_type_pack_bindings: HashMap<Name, TypePackId>,

  pub refinements: RefinementMap,
  pub lvalue_types: DenseHashMap<*const Def, TypeId>,
  pub rvalue_refinements: DenseHashMap<*const Def, TypeId>,

  pub globals_to_warn: DenseHashSet<String>,

  pub type_alias_type_parameters: HashMap<Name, TypeId>,
  pub type_alias_type_pack_parameters: HashMap<Name, TypePackId>,

  pub interior_free_types: Option<Vec<TypeId>>,
  pub interior_free_type_packs: Option<Vec<TypePackId>>,

  pub invalid_type_aliases: DenseHashMap<String, Location>,
}

unsafe impl Send for Scope {}
unsafe impl Sync for Scope {}
