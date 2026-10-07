//! `scope` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  functions::get_base_symbol::get_base_symbol,
  records::{
    arena_handle::alias_ref,
    binding::Binding,
    rejected_aliases::RejectedAliases,
    scope::Scope,
    scope_registry::{intern_scope, resolve_scope},
    symbol::Symbol,
    type_fun::TypeFun,
    type_level::TypeLevel,
  },
  type_aliases::{
    collections::HashMap, def_id_def::DefId, name_type::Name, scope_ptr_type::ScopePtr,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl Scope {
  pub fn add_builtin_type_binding(&mut self, name: &str, ty_fun: &TypeFun) {
    self
      .exported_type_bindings
      .insert(name.to_string(), ty_fun.clone());
    self.builtin_type_names.insert(name.to_string());
  }
}

impl Scope {
  pub fn find_narrowest_scope_containing(&mut self, location: Location) -> *mut Scope {
    let mut best_scope = self as *mut Scope;

    loop {
      let mut did_narrow = false;
      let children = &alias_ref(best_scope).children;

      for &child_id in children {
        let Some(child) = resolve_scope(child_id) else {
          continue;
        };
        if child.location.encloses(&location) {
          best_scope = child as *const Scope as *mut Scope;
          did_narrow = true;
          break;
        }
      }

      if !did_narrow || alias_ref(best_scope).children.is_empty() {
        break;
      }
    }

    best_scope
  }
}

impl Scope {
  pub fn inherit_assignments(&mut self, child_scope: &ScopePtr) {
    // cpp `lvalueTypes[k] = a`：operator[] 是覆盖写，已存在的键必须刷新
    for (k, a) in child_scope.lvalue_types.iter() {
      *self.lvalue_types.get_or_insert(*k) = *a;
    }
  }
}

impl Scope {
  // Updates the `this` scope with the refinements from the `childScope`
  // excluding ones that don't exist in `this` (Scope.cpp:226).
  pub fn inherit_refinements(&mut self, child_scope: &ScopePtr) {
    for (&k, &a) in &child_scope.rvalue_refinements {
      if self.lookup_def_id(k).is_some() {
        *self.rvalue_refinements.get_or_insert(k) = a;
      }
    }

    for (k, a) in &child_scope.refinements {
      let symbol: Symbol = get_base_symbol(k);
      if self.lookup_symbol(symbol).is_some() {
        self.refinements.insert(k.clone(), *a);
      }
    }
  }
}

impl Scope {
  /// cpp `scope->invalidTypeAliases.try_insert(name, location)`（ConstraintSolver
  /// 派发路径）的收口写入：`Scope` 经 `ScopePtr = Arc<Scope>` 共享，故写入落在
  /// 内部可变登记表 [`RejectedAliases`] 上（旧写法 `from_ref(self).cast_mut()`
  /// 经共享引用解写属 UB），本方法随之转为 safe、不再含 unsafe 块。
  pub fn record_invalid_type_alias(&self, name: String, location: Location) {
    self.invalid_type_aliases.record(name, location);
  }

  pub fn is_invalid_type_alias(&self, name: &str) -> Option<Location> {
    // r7-rc-5（承 r7-rc-4）：DenseHashMap<String, _> 已有 &str 借用查询口，
    // an2 票的循环外键物化（`let key = String::from(name)`）整体删除，零分配查询。
    let mut scope: Option<&Scope> = Some(self);
    while let Some(current_scope) = scope {
      if let Some(loc) = current_scope.invalid_type_aliases.find(name) {
        return Some(loc);
      }

      scope = current_scope.parent.and_then(resolve_scope);
    }
    None
  }
}

impl Scope {
  pub fn linear_search_for_binding(
    &self,
    name: &str,
    traverse_scope_chain: bool,
  ) -> Option<Binding> {
    let mut scope: Option<&Scope> = Some(self);

    while let Some(current_scope) = scope {
      for (symbol, binding) in &current_scope.bindings {
        if symbol.name() == name {
          return Some(binding.clone());
        }
      }

      scope = current_scope.parent.and_then(resolve_scope);

      if !traverse_scope_chain {
        break;
      }
    }

    None
  }
}

impl Scope {
  pub fn linear_search_for_binding_pair(
    &self,
    name: &str,
    traverse_scope_chain: bool,
  ) -> Option<(Symbol, Binding)> {
    let mut scope: Option<&Scope> = Some(self);

    while let Some(current_scope) = scope {
      for (symbol, binding) in &current_scope.bindings {
        if symbol.name() == name {
          return Some((symbol.clone(), binding.clone()));
        }
      }

      if !traverse_scope_chain {
        break;
      }

      scope = current_scope.parent.and_then(resolve_scope);
    }

    None
  }
}

impl Scope {
  /// 沿 parent 链查找 sym，返回（绑定，定义处作用域）。
  /// cpp/Scope.cpp:32 用 const_cast 取 &mut；本实现真只读（&self），无 unsafe。
  pub fn lookup_ex_symbol(&self, sym: Symbol) -> Option<(&Binding, &Scope)> {
    let mut cur = self;
    loop {
      if let Some(binding) = cur.bindings.get(&sym) {
        return Some((binding, cur));
      }
      cur = cur.parent.and_then(resolve_scope)?;
    }
  }
}

impl Scope {
  pub fn lookup_imported_type(&self, module_alias: &Name, name: &Name) -> Option<TypeFun> {
    let mut scope: Option<&Scope> = Some(self);
    while let Some(current_scope) = scope {
      if let Some(imported_bindings) = current_scope.imported_type_bindings.get(module_alias)
        && let Some(type_fun) = imported_bindings.get(name)
      {
        return Some(type_fun.clone());
      }
      scope = current_scope.parent.and_then(resolve_scope);
    }
    None
  }
}

impl Scope {
  pub fn lookup_pack(&self, name: &Name) -> Option<TypePackId> {
    let mut scope: &Scope = self;
    loop {
      if let Some(type_pack_id) = scope.private_type_pack_bindings.get(name) {
        return Some(*type_pack_id);
      }

      {
        let parent = scope.parent.and_then(resolve_scope)?;
        scope = parent;
      }
    }
  }
}

impl Scope {
  /// `std::optional<TypeId> Scope::lookupRValueRefinementType(DefId def) const`
  /// (Scope.cpp:87-96).
  pub fn lookup_r_value_refinement_type(&self, def: DefId) -> Option<TypeId> {
    let mut current: Option<&Scope> = Some(self);
    while let Some(scope) = current {
      if let Some(ty) = scope.rvalue_refinements.find(&def) {
        return Some(*ty);
      }

      current = scope.parent.and_then(resolve_scope);
    }

    None
  }
}

impl Scope {
  pub fn lookup_symbol(&self, sym: Symbol) -> Option<TypeId> {
    self
      .lookup_ex_symbol(sym)
      .map(|(binding, _)| binding.type_id)
  }

  /// `std::optional<TypeId> Scope::lookup(DefId def) const` (Scope.cpp:98-110).
  pub fn lookup_def_id(&self, def: DefId) -> Option<TypeId> {
    let mut current: Option<&Scope> = Some(self);
    while let Some(scope) = current {
      if let Some(ty) = scope.rvalue_refinements.find(&def) {
        return Some(*ty);
      }
      if let Some(ty) = scope.lvalue_types.find(&def) {
        return Some(*ty);
      }

      current = scope.parent.and_then(resolve_scope);
    }

    None
  }
}

impl Scope {
  pub fn lookup_type(&self, name: &str) -> Option<TypeFun> {
    let mut current_scope: &Scope = self;
    loop {
      if let Some(type_fun) = current_scope.exported_type_bindings.get(name) {
        return Some(type_fun.clone());
      }

      if let Some(type_fun) = current_scope.private_type_bindings.get(name) {
        return Some(type_fun.clone());
      }

      current_scope = current_scope.parent.and_then(resolve_scope)?;
    }
  }
}

impl Scope {
  pub fn lookup_unrefined_type(&self, def: DefId) -> Option<TypeId> {
    let mut current: Option<&Scope> = Some(self);
    while let Some(scope) = current {
      if let Some(ty) = scope.lvalue_types.find(&def) {
        return Some(*ty);
      }

      current = scope.parent.and_then(resolve_scope);
    }

    None
  }
}

// C++ `Scope::Scope(const ScopePtr& parent, int sub_level = 0)`
// (`Analysis/src/Scope.cpp`): a child scope inherits its parent's return type
// and an incremented type level, and value-initializes every container. The
// `DenseHash*` empty-key sentinels match the in-class initializers in
// `Analysis/include/Luau/Scope.h` (`{nullptr}`, `{""}`, `{{}}`).
//
// 字段初始化复用根 scope 构造 [`Scope::scope_type_pack_id`] 的同一份全字段
// value-initialization（cpp 两个 ctor 共享同一组类内初始化器），再覆写三个
// 差异字段（parent/level）。

impl Scope {
  /// 子 scope 构造点：`parent` 经 [`intern_scope`] 幂等入表（cpp 中把
  /// `ScopePtr` 交给子 scope 的强引用 parent 即自动保活，句柄态等价物见
  /// scope_registry 模块契约 1），故任意 `Arc<Scope>` 均可安全作为父传入。
  pub fn new(parent: &ScopePtr, sub_level: i32) -> Self {
    let mut this = Scope::scope_type_pack_id(parent.return_type);
    this.parent = Some(intern_scope(parent));

    let mut level = parent.level.incr();
    level.sub_level = sub_level;
    this.level = level;

    this
  }
}

// C++ `Scope::Scope(TypePackId returnType)` (`Analysis/src/Scope.cpp:10`):
// a root scope with no parent, the given return type, and a default
// `TypeLevel{}`. Every other member uses its in-class initializer (the
// `DenseHash*` empty-key sentinels from `Analysis/include/Luau/Scope.h`).

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
      lvalue_types: DenseHashMap::default(),
      rvalue_refinements: DenseHashMap::default(),
      globals_to_warn: DenseHashSet::default(),
      type_alias_type_parameters: HashMap::new(),
      type_alias_type_pack_parameters: HashMap::new(),
      interior_free_types: None,
      interior_free_type_packs: None,
      invalid_type_aliases: RejectedAliases::default(),
    }
  }
}

impl Scope {
  pub fn should_warn_global(&self, name: &str) -> bool {
    let mut current = Some(self);
    while let Some(scope) = current {
      if scope.globals_to_warn.contains_str(name) {
        return true;
      }
      current = scope.parent.and_then(resolve_scope);
    }
    false
  }
}
