//! `subtyping_environment` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::LUAU_ASSERT;

use crate::{
  functions::follow_type::follow,
  methods::subtyping_bind_generic::{
    dense_hash_map_find_mut_no_default, dense_hash_map_find_no_default,
  },
  records::{
    apply_mapped_generics::ApplyMappedGenerics, arena_handle::Handle, builtin_types::BuiltinTypes,
    generic_bounds::GenericBounds, internal_error_reporter::InternalErrorReporter,
    substitution::Substitution, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, txn_log::TxnLog, type_arena::TypeArena,
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
    ice_reporter: *mut InternalErrorReporter,
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
          unsafe { (*self.parent).contains_mapped_pack(tp) }
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
      return unsafe { (*self.parent).contains_mapped_type(ty) };
    }

    false
  }
}

impl SubtypingEnvironment {
  /// # Safety
  /// - `self.parent` 及其父链为构造期接线的 `*mut SubtypingEnvironment` 环境层级裸指针：
  ///   可空（空则不递归），非空者指向比本环境长寿的外层作用域环境。
  /// - `ice_reporter` 必须非 null 且指向存活的 `InternalErrorReporter`——对应 C++
  ///   `getMappedTypeBounds(TypeId, InternalErrorReporter&)` 的引用形参，调用方从不传 null。
  /// - `ty` 经 `follow` 解引用，须指向类型 arena 中存活节点。
  pub unsafe fn get_mapped_type_bounds(
    &mut self,
    ty: TypeId,
    ice_reporter: *mut InternalErrorReporter,
  ) -> &mut GenericBounds {
    let ty = follow(ty);
    if let Some(bounds) = dense_hash_map_find_mut_no_default(&mut self.mapped_generics, &ty)
      && !bounds.is_empty()
    {
      // 链上 `!bounds.is_empty()` 蕴含 last_mut() 命中 Some。
      return bounds.last_mut().expect("链上 !bounds.is_empty() 蕴含非空");
    }

    if !self.parent.is_null() {
      // Safety: 上方 `!self.parent.is_null()` 已保证父环境句柄非空；它是构造期接线的
      // 环境父链裸指针，指向比 `&mut self` 长寿的外层环境，故递归返回的 `&mut GenericBounds`
      // 在此存活有效。递归调用的 unsafe fn 契约（父链/`ice_reporter`/`ty`）与原调用一致。
      return unsafe { (*self.parent).get_mapped_type_bounds(ty, ice_reporter) };
    }

    LUAU_ASSERT!(false);
    // Safety: `ice_reporter` 依函数级契约非 null 且指向存活 `InternalErrorReporter`（C++
    // 侧为引用形参，恒非空）；`ice_string` 只上报一条诊断消息并 panic 发散，不产生并存别名。
    unsafe {
      (*ice_reporter).ice_string("Trying to access bounds for a type with no in-scope bounds");
    }
    unreachable!()
  }
}

impl SubtypingEnvironment {
  pub fn lookup_generic_pack(&self, tp: TypePackId) -> LookupResult {
    let result = self.mapped_generic_packs.lookup_generic_pack(tp);
    if result.get_if::<TypePackId>().is_some() {
      result
    } else if !self.parent.is_null() {
      unsafe { (*self.parent).lookup_generic_pack(tp) }
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
      return unsafe { (*self.parent).try_find_substitution(ty) };
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
      return unsafe { (*self.parent).try_find_subtyping_result(sub_and_super) };
    }

    None
  }
}
