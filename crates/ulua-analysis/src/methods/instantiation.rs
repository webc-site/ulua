//! `instantiation` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::{from_ref, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{as_mutable_type::as_mutable_type_id, get_type},
  macros::{substitution_entry::substitution_entry, substitution_vtable},
  records::{
    arena_handle::{Handle, alias, alias_nn_opt, alias_nn_ref, alias_ref},
    builtin_types::BuiltinTypes,
    extern_type::ExternType,
    function_type::FunctionType,
    instantiation::Instantiation,
    replace_generics::ReplaceGenerics,
    scope::Scope,
    substitution::Substitution,
    txn_log::TxnLog,
    type_arena::TypeArena,
    type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Instantiation {
  pub(crate) fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    let ftv = alias_ref(self.base.base.log).txn_log_get_mutable::<FunctionType, TypeId>(ty);
    LUAU_ASSERT!(ftv.is_some());
    let ftv = alias_nn_ref(ftv.expect("LUAU_ASSERT 判 FunctionType pending 命中"));

    let mut clone = FunctionType::function_type_new(
      ftv.arg_types,
      ftv.ret_types,
      ftv.definition.clone(),
      ftv.has_self,
    );
    clone.level = self.level;
    clone.magic = ftv.magic.clone();
    clone.tags = ftv.tags.clone();
    clone.arg_names = ftv.arg_names.clone();
    clone.is_deprecated_function = ftv.is_deprecated_function;
    clone.deprecated_info = ftv.deprecated_info.clone();
    clone.is_checked_function = ftv.is_checked_function;

    let result = self.base.add_type(clone);

    self.reusable_replace_generics.reset_state(
      self.base.base.log,
      self.base.wired_arena_handle(),
      self.builtin_types,
      self.level,
      self.scope_opt_ref(),
      ftv.generics.clone(),
      ftv.generic_packs.clone(),
    );

    let result = self
      .reusable_replace_generics
      .substitute_type_id(result)
      .unwrap_or(result);

    alias(as_mutable_type_id(result)).documentation_symbol =
      alias_ref(ty).documentation_symbol.clone();

    result
  }

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    LUAU_ASSERT!(false);
    tp
  }
}

impl Instantiation {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    // 对齐 cpp Instantiation.cpp:49-55：FunctionType 读取改走 log 的 pending 态
    // （txn_log_get_mutable）；ExternType 在 cpp 中本就是 plain get，保持不变。
    let log = self.base.base.log;
    let ft = alias_ref(log).txn_log_get_mutable::<FunctionType, TypeId>(ty);
    if ft.is_some() {
      return true;
    }

    get_type::get::<ExternType>(ty).is_some()
  }
}

// Source: `Analysis/include/Luau/Instantiation.h` (Instantiation.h:66-73, hand-ported)

// Instantiation 退化形态：pack 侧三槽全为 C++ 基类默认（isDirty=false、clean 恒等
// 透传、ignoreChildren=false），见 substitution_vtable 模块文档的统一安全论证。
substitution_vtable!(degen_tp, Instantiation, ic = ignore_children);
impl Instantiation {
  /// C++ `Instantiation(const TxnLog* log, TypeArena* arena, NotNull<BuiltinTypes> builtinTypes,
  /// TypeLevel level, Scope* scope) : Substitution(log, arena), builtinTypes(builtinTypes),
  /// level(level), scope(scope), reusableReplaceGenerics(log, arena, builtinTypes, level, scope, {}, {})`.
  pub fn instantiation_new(
    log: *const TxnLog,
    arena: Option<Handle<TypeArena>>,
    builtin_types: Handle<BuiltinTypes>,
    level: TypeLevel,
    scope: Option<&Scope>,
  ) -> Self {
    // 边界收口：`scope` 在 cpp 即可空 `Scope*`（旧 solver 传 nullptr），记录
    // 字段保持裸指针布局，`None` 折叠为 null 哨兵，链内传递均为 `Option<&Scope>`。
    let scope_raw = scope.map(|s| from_ref(s).cast_mut());
    Instantiation {
      base: Substitution::substitution_new(log, arena),
      builtin_types,
      level,
      scope: scope_raw.unwrap_or(null_mut()),
      reusable_replace_generics: ReplaceGenerics::replace_generics_new(
        log,
        arena,
        builtin_types,
        level,
        scope,
        Vec::new(),
        Vec::new(),
      ),
    }
  }

  substitution_entry!(id, pack);
}

impl Instantiation {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    let log = self.base.base.log;
    let ftv = alias_ref(log).txn_log_get_mutable::<FunctionType, TypeId>(ty);
    if let Some(ftv) = alias_nn_opt(ftv) {
      if ftv.has_no_free_or_generic_types {
        return false;
      }
      return true;
    }
    false
  }

  pub fn is_dirty_type_pack_id(&self, _tp: TypePackId) -> bool {
    false
  }
}

impl Instantiation {
  pub fn reset_state(
    &mut self,
    log: *const TxnLog,
    arena: Handle<TypeArena>,
    builtin_types: Handle<BuiltinTypes>,
    level: TypeLevel,
    scope: Option<&Scope>,
  ) {
    let scope_raw = scope.map(|s| from_ref(s).cast_mut());
    Substitution::reset_state(&mut self.base, log, arena);

    self.builtin_types = builtin_types;
    self.level = level;
    self.scope = scope_raw.unwrap_or(null_mut());

    self.reusable_replace_generics.reset_state(
      log,
      arena,
      builtin_types,
      level,
      scope,
      Vec::new(),
      Vec::new(),
    );
  }
}
