//! `replace_generics` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::{from_ref, null_mut};

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::table_state::TableState,
  functions::{get_mutable_type, get_type},
  macros::{substitution_entry::substitution_entry, substitution_vtable},
  records::{
    arena_handle::{Handle, alias_nn_opt, alias_ref},
    builtin_types::BuiltinTypes,
    extern_type::ExternType,
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    function_type::FunctionType,
    generic_type::GenericType,
    generic_type_pack::GenericTypePack,
    replace_generics::ReplaceGenerics,
    scope::Scope,
    substitution::Substitution,
    table_type::TableType,
    txn_log::TxnLog,
    type_arena::TypeArena,
    type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ReplaceGenerics {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    LUAU_ASSERT!(self.is_dirty_type_id(ty));

    let log = self.base.base.log;
    let level = self.level;
    let scope = self.scope;
    // Safety: `self.builtin_types` 对应 C++ `NotNull<BuiltinTypes>`，构造期
    // 接线、非空且比本替换器长寿；取共享引用只读 never/unknown 两个内置
    // TypeId，不写穿，单线程串行下无并发可变句柄。函数头一次借用，供
    // 下方各 else 分支共用。
    let builtins = self.builtin_types.get();

    if fflag::LuauReplacerIsSolverAgnostic.get() {
      let ttv = alias_ref(log).txn_log_get_mutable::<TableType, TypeId>(ty);
      if let Some(ttv) = alias_nn_opt(ttv) {
        let mut clone =
          TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
            &ttv.props,
            ttv.indexer,
            level,
            scope,
            TableState::Free,
          );
        clone.definition_module_name = ttv.definition_module_name.clone();
        clone.definition_location = ttv.definition_location;
        self.base.add_type(clone)
      } else {
        // arena->freshType(builtinTypes, scope, level)
        let free_type = FreeType {
          scope,
          level,
          lower_bound: builtins.never_type,
          upper_bound: builtins.unknown_type,
          ..FreeType::default()
        };
        self.base.add_type(free_type)
      }
    } else {
      let ttv = alias_ref(log).txn_log_get_mutable::<TableType, TypeId>(ty);
      if let Some(ttv) = alias_nn_opt(ttv) {
        let mut clone =
          TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
            &ttv.props,
            ttv.indexer,
            level,
            scope,
            TableState::Free,
          );
        clone.definition_module_name = ttv.definition_module_name.clone();
        clone.definition_location = ttv.definition_location;
        self.base.add_type(clone)
      } else if fflag::LuauSolverV2.get() {
        let free_type = FreeType {
          scope,
          lower_bound: builtins.never_type,
          upper_bound: builtins.unknown_type,
          ..FreeType::default()
        };
        let res = self.base.add_type(free_type);
        if let Some(ft) = get_mutable_type::get_mutable::<FreeType>(res) {
          ft.level = level;
        }
        res
      } else {
        // arena->freshType(builtinTypes, scope, level)
        let free_type = FreeType {
          scope,
          level,
          lower_bound: builtins.never_type,
          upper_bound: builtins.unknown_type,
          ..FreeType::default()
        };
        self.base.add_type(free_type)
      }
    }
  }

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    LUAU_ASSERT!(self.is_dirty_type_pack_id(tp));
    let mut pack = FreeTypePack::new(self.level);
    pack.scope = self.scope;
    self.base.add_type_pack(pack)
  }
}

impl ReplaceGenerics {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    // 对齐 cpp Instantiation.cpp:109-129：FunctionType 读取改走 log 的 pending 态
    // （txn_log_get_mutable），与 is_dirty_type_id 的 log 感知保持一致；
    // ExternType 一侧 cpp 本就是 plain get，保持不变。
    let log = self.base.base.log;
    let ftv = alias_ref(log).txn_log_get_mutable::<FunctionType, TypeId>(ty);
    if let Some(ftv_ref) = alias_nn_opt(ftv) {
      if ftv_ref.has_no_free_or_generic_types {
        return true;
      }

      return (!self.generics.is_empty() || !self.generic_packs.is_empty())
        && (ftv_ref.generics == self.generics)
        && (ftv_ref.generic_packs == self.generic_packs);
    }

    get_type::get::<ExternType>(ty).is_some()
  }
}

impl ReplaceGenerics {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    let log = self.base.base.log;

    let ttv = alias_ref(log).txn_log_get_mutable::<TableType, TypeId>(ty);
    if let Some(ttv) = alias_nn_opt(ttv) {
      return ttv.state == TableState::Generic;
    }

    let gtv = alias_ref(log).txn_log_get_mutable::<GenericType, TypeId>(ty);
    gtv.is_some() && self.generics.contains(&ty)
  }

  pub fn is_dirty_type_pack_id(&self, tp: TypePackId) -> bool {
    let log = self.base.base.log;
    // 对齐 cpp Instantiation.cpp:141-147：仅当 log 的 pending 态命中 GenericTypePack 时
    // 才做 genericPacks 成员判断，否则直接视为 not-dirty。
    let gtp = alias_ref(log).txn_log_get_mutable::<GenericTypePack, TypePackId>(tp);
    gtp.is_some() && self.generic_packs.contains(&tp)
  }
}

// Source: `Analysis/include/Luau/Instantiation.h` (Instantiation.h:21-37, hand-ported)

// C++ 未覆写 `ignoreChildren(TypePackId)`，保持基类默认 false（pack 侧三槽中
// isDirty/clean 仍为真实转发，见 substitution_vtable 模块文档的统一安全论证）。
substitution_vtable!(false_tp, ReplaceGenerics, ic = ignore_children);
impl ReplaceGenerics {
  /// C++ `ReplaceGenerics(const TxnLog* log, TypeArena* arena, NotNull<BuiltinTypes> builtinTypes,
  /// TypeLevel level, Scope* scope, const std::vector<TypeId>& generics,
  /// const std::vector<TypePackId>& genericPacks) : Substitution(log, arena), ...`.
  pub fn replace_generics_new(
    log: *const TxnLog,
    arena: Option<Handle<TypeArena>>,
    builtin_types: Handle<BuiltinTypes>,
    level: TypeLevel,
    scope: Option<&Scope>,
    generics: Vec<TypeId>,
    generic_packs: Vec<TypePackId>,
  ) -> Self {
    // 边界收口：`scope` 记录字段保持裸指针布局（cpp 可空 `Scope*`），入口引用在此还原。
    let scope_raw = scope.map(|s| from_ref(s).cast_mut());
    ReplaceGenerics {
      base: Substitution::substitution_new(log, arena),
      builtin_types,
      level,
      scope: scope_raw.unwrap_or(null_mut()),
      generics,
      generic_packs,
    }
  }

  substitution_entry!(id, pack);
}

impl ReplaceGenerics {
  pub fn reset_state(
    &mut self,
    log: *const TxnLog,
    arena: Handle<TypeArena>,
    builtin_types: Handle<BuiltinTypes>,
    level: TypeLevel,
    scope: Option<&Scope>,
    generics: Vec<TypeId>,
    generic_packs: Vec<TypePackId>,
  ) {
    let scope_raw = scope.map(|s| from_ref(s).cast_mut());
    self.base.reset_state(log, arena);

    self.builtin_types = builtin_types;

    self.level = level;
    self.scope = scope_raw.unwrap_or(null_mut());

    self.generics = generics;
    self.generic_packs = generic_packs;
  }
}
