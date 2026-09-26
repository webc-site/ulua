//! C++ `Scope::Scope(const ScopePtr& parent, int sub_level = 0)`
//! (`Analysis/src/Scope.cpp`): a child scope inherits its parent's return type
//! and an incremented type level, and value-initializes every container. The
//! `DenseHash*` empty-key sentinels match the in-class initializers in
//! `Analysis/include/Luau/Scope.h` (`{nullptr}`, `{""}`, `{{}}`).
use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{scope::Scope, scope_registry::intern_scope},
  type_aliases::{collections::HashMap, scope_ptr_type::ScopePtr},
};
impl Scope {
  /// 子 scope 构造点：`parent` 经 [`intern_scope`] 幂等入表（cpp 中把
  /// `ScopePtr` 交给子 scope 的强引用 parent 即自动保活，句柄态等价物见
  /// scope_registry 模块契约 1），故任意 `Arc<Scope>` 均可安全作为父传入。
  pub fn new(parent: &ScopePtr, sub_level: i32) -> Self {
    let mut level = parent.level.incr();
    level.sub_level = sub_level;

    Scope {
      parent: Some(intern_scope(parent)),
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
      lvalue_types: DenseHashMap::default(),
      rvalue_refinements: DenseHashMap::default(),
      globals_to_warn: DenseHashSet::default(),
      type_alias_type_parameters: HashMap::new(),
      type_alias_type_pack_parameters: HashMap::new(),
      interior_free_types: None,
      interior_free_type_packs: None,
      invalid_type_aliases: DenseHashMap::default(),
    }
  }
}
