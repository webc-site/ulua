//! `instantiation_2` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::from_ref;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  functions::{follow_type, get_type, get_type_pack},
  macros::{substitution_entry::substitution_entry, substitution_vtable},
  records::{
    arena_handle::Handle, extern_type::ExternType, free_type::FreeType,
    function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    instantiation_2::Instantiation2, never_type::NeverType, scope::Scope,
    substitution::Substitution, subtyping::Subtyping, txn_log::TxnLog, type_arena::TypeArena,
    unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Instantiation2 {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    LUAU_ASSERT!(!self.subtyping.is_null() && !self.scope.is_null());
    LUAU_ASSERT!(get_type::get::<GenericType>(ty).is_some());

    // SAFETY: generic_substitutions 在 Instantiation2 存活期内有效。
    let subst_ty = follow_type::follow(
      *self
        .generic_substitutions
        .find(&ty)
        .expect("TypeId not found in generic_substitutions"),
    );
    let ft = get_type::get::<FreeType>(subst_ty);
    LUAU_ASSERT!(ft.is_some());
    // C++ `LUAU_ASSERT(ft)` 必命中，紧邻断言蕴含 Some。
    let ft = ft.expect("紧邻 LUAU_ASSERT(ft.is_some()) 蕴含");

    let lower_bound = ft.lower_bound;
    let upper_bound = ft.upper_bound;

    let res = if get_type::get::<NeverType>(follow_type::follow(lower_bound)).is_some() {
      upper_bound
    } else if get_type::get::<UnknownType>(follow_type::follow(upper_bound)).is_some() {
      lower_bound
    } else {
      // SAFETY: subtyping 在 Instantiation2 存活期内有效（上方已断言非空）。
      let r = unsafe {
        (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
          lower_bound,
          upper_bound,
          self.scope_ref(),
        )
      };
      if r.is_subtype {
        lower_bound
      } else {
        upper_bound
      }
    };

    self.base.dont_traverse_into_type_id(res);
    res
  }

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let res = self
      .generic_pack_substitutions
      .find(&tp)
      .expect("TypePackId not found in generic_pack_substitutions");
    LUAU_ASSERT!(!res.is_null());
    let cleaned = *res;
    self.base.dont_traverse_into_type_pack_id(cleaned);
    cleaned
  }
}

impl Instantiation2 {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    if get_type::get::<ExternType>(ty).is_some() {
      return true;
    }

    if let Some(ftv) = get_type::get::<FunctionType>(ty).as_ref() {
      if ftv.has_no_free_or_generic_types {
        return false;
      }

      for &generic in &ftv.generics {
        if self.generic_substitutions.find(&generic).is_some() {
          return true;
        }
      }

      for &generic in &ftv.generic_packs {
        if self.generic_pack_substitutions.find(&generic).is_some() {
          return true;
        }
      }
    }

    false
  }
}

// C++ 未覆写 `ignoreChildren(TypePackId)`，保持基类默认 false（pack 侧三槽中
// isDirty/clean 仍为真实转发，见 substitution_vtable 模块文档的统一安全论证）。
substitution_vtable!(false_tp, Instantiation2, ic = ignore_children);
impl Instantiation2 {
  pub fn instantiation_2_type_arena_dense_hash_map_type_id_type_id_dense_hash_map_type_pack_id_type_pack_id_not_null_subtyping_not_null_scope(
    arena: Handle<TypeArena>,
    generic_substitutions: DenseHashMap<TypeId, TypeId>,
    generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId>,
    subtyping: *mut Subtyping,
    scope: &Scope,
  ) -> Self {
    Instantiation2 {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      generic_substitutions,
      generic_pack_substitutions,
      subtyping,
      // 边界收口：字段保持 cpp `NotNull<Scope>` 的裸指针布局（不改记录布局），
      // 入口引用仅在此处一次性还原地址，链内传递均为 `&Scope`。
      scope: from_ref(scope).cast_mut(),
    }
  }

  substitution_entry!(id, pack);
}

impl Instantiation2 {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    let gt = get_type::get::<GenericType>(ty);
    if gt.is_none() {
      return false;
    }
    { self.generic_substitutions.find(&ty) }.is_some()
  }

  pub fn is_dirty_type_pack_id(&self, tp: TypePackId) -> bool {
    let generic_pack = get_type_pack::get::<GenericTypePack>(tp);
    generic_pack.is_some() && self.generic_pack_substitutions.find(&tp).is_some()
  }

  /// C++ `NotNull<Scope> scope` 成员的收口访问器（同 `Constraint::scope_ref`）：
  /// 字段保持裸指针布局，构造期由 `NotNull` 语义建立、生命周期覆盖 Instantiation2 全程。
  pub(crate) fn scope_ref(&self) -> &Scope {
    // SAFETY: 构造契约恒非空。
    unsafe { &*self.scope }
  }
}
