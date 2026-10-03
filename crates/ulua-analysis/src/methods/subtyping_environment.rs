//! `subtyping_environment` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::LUAU_ASSERT;

use crate::{
  functions::follow_type::follow,
  methods::subtyping_bind_generic::{
    dense_hash_map_find_mut_no_default, dense_hash_map_find_no_default,
  },
  records::{
    apply_mapped_generics::ApplyMappedGenerics,
    arena_handle::{Handle, alias, alias_ref},
    builtin_types::BuiltinTypes,
    generic_bounds::GenericBounds,
    internal_error_reporter::InternalErrorReporter,
    substitution::Substitution,
    subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult,
    txn_log::TxnLog,
    type_arena::TypeArena,
  },
  type_aliases::{lookup_result::LookupResult, type_id::TypeId, type_pack_id::TypePackId},
};

impl SubtypingEnvironment {
  /// C++ `SubtypingEnvironment::applyMappedGenerics` (`Subtyping.cpp:504-513`):
  ///
  /// ```cpp
  /// ApplyMappedGenerics amg{builtinTypes, arena, *this, iceReporter};
  /// return amg.substitute(ty);
  /// ```
  ///
  /// `ApplyMappedGenerics` extends `Substitution` and inherits `substitute`,
  /// whose traversal virtual-dispatches into the overridden `isDirty` /
  /// `clean` / `ignoreChildren`. The Rust `ApplyMappedGenerics` now embeds
  /// `base: Substitution` and installs those overrides into the
  /// `SubstitutionVtable` from its `substitute_type_id` wrapper.
  pub fn apply_mapped_generics(
    &mut self,
    builtin_types: Handle<BuiltinTypes>,
    arena: Handle<TypeArena>,
    ty: TypeId,
    ice_reporter: Handle<InternalErrorReporter>,
  ) -> Option<TypeId> {
    let mut amg = ApplyMappedGenerics {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      builtin_types,
      arena,
      ice_reporter,
      env: self as *mut SubtypingEnvironment,
    };
    amg.substitute_type_id(ty)
  }
}

impl SubtypingEnvironment {
  pub fn contains_mapped_pack(&self, tp: TypePackId) -> bool {
    let lookup_result: LookupResult = self.lookup_generic_pack(tp);
    match lookup_result {
      LookupResult::V0(_) => true,
      _ => {
        if !self.parent.is_null() {
          alias_ref(self.parent).contains_mapped_pack(tp)
        } else {
          false
        }
      }
    }
  }
}

impl SubtypingEnvironment {
  pub fn contains_mapped_type(&self, ty: TypeId) -> bool {
    let ty = follow(ty);
    if let Some(bounds) = dense_hash_map_find_no_default(&self.mapped_generics, &ty)
      && !bounds.is_empty()
    {
      return true;
    }

    if !self.parent.is_null() {
      return alias_ref(self.parent).contains_mapped_type(ty);
    }

    false
  }
}

impl SubtypingEnvironment {
  /// 查找 `ty`（经 `follow`）在父链中对应的泛型约束边界。
  ///
  /// 前提由 `SubtypingEnvironment` 构造不变量保证：`self.parent` 为空，或指向
  /// 比本环境长寿的外层作用域环境；`ice_reporter` 非空（`Handle` 类型编码）且
  /// 在本调用返回前存活。`ty` 为类型 arena 中存活节点（与任意 `TypeId` 用法同契约）。
  pub(crate) fn get_mapped_type_bounds(
    &mut self,
    ty: TypeId,
    ice_reporter: Handle<InternalErrorReporter>,
  ) -> &mut GenericBounds {
    let ty = follow(ty);
    if let Some(bounds) = dense_hash_map_find_mut_no_default(&mut self.mapped_generics, &ty)
      && !bounds.is_empty()
    {
      // 链上 `!bounds.is_empty()` 蕴含 last_mut() 命中 Some。
      return bounds.last_mut().expect("链上 !bounds.is_empty() 蕴含非空");
    }

    if !self.parent.is_null() {
      // `self.parent` 非空由构造不变量保证；`alias()` 将其转为 `&'static mut`，
      // 父链严格向外指且无自别名，递归返回的借用覆盖本次调用。
      return alias(self.parent).get_mapped_type_bounds(ty, ice_reporter);
    }

    LUAU_ASSERT!(false);
    // `ice_reporter` 为 Handle（非空由类型编码，对应 C++ 引用形参）；`ice_string`
    // 只上报一条诊断消息并 panic 发散，不产生并存别名。
    ice_reporter
      .get()
      .ice_string("Trying to access bounds for a type with no in-scope bounds");
    unreachable!()
  }
}

impl SubtypingEnvironment {
  pub fn lookup_generic_pack(&self, tp: TypePackId) -> LookupResult {
    let result = self.mapped_generic_packs.lookup_generic_pack(tp);
    if result.get_if::<TypePackId>().is_some() {
      result
    } else if !self.parent.is_null() {
      alias_ref(self.parent).lookup_generic_pack(tp)
    } else {
      result
    }
  }
}

impl SubtypingEnvironment {
  pub fn try_find_substitution(&self, ty: TypeId) -> Option<TypeId> {
    if let Some(it) = self.substitutions.find(&ty) {
      return Some(*it);
    }

    if !self.parent.is_null() {
      return alias_ref(self.parent).try_find_substitution(ty);
    }

    None
  }
}

impl SubtypingEnvironment {
  pub fn try_find_subtyping_result(
    &self,
    sub_and_super: (TypeId, TypeId),
  ) -> Option<&SubtypingResult> {
    if let Some(it) = self.seen_set_cache.find(&sub_and_super) {
      return Some(it);
    }

    if !self.parent.is_null() {
      return alias_ref(self.parent).try_find_subtyping_result(sub_and_super);
    }

    None
  }
}
