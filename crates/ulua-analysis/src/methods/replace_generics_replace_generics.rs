//! Source: `Analysis/include/Luau/Instantiation.h` (Instantiation.h:21-37, hand-ported)
use alloc::vec::Vec;

use crate::{
  macros::substitution_entry::substitution_entry,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, replace_generics::ReplaceGenerics,
    scope::Scope, substitution::Substitution, tarjan::SubstitutionVtable, txn_log::TxnLog,
    type_arena::TypeArena, type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

// 以下 `replace_generics_*` 均为 `SubstitutionVtable` 的 fn 指针槽：由
// `install_substitution_vtable` 把 `self as *mut ReplaceGenerics as *mut ()`
// 存入 `vtable.owner`，仅在 substitute_* 遍历期、该对象被调用栈独占持有时回调。
// 单线程、遍历期裸指针重入不变量保证回调窗口内 owner 回转不产生并存别名。

fn replace_generics_is_dirty_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 为 install_substitution_vtable 装入的存活 ReplaceGenerics 句柄
  // （非空、对齐：由 self 地址直接转来）；is_dirty_type_id 取 &self 只读借用仅
  // 覆盖本次回调，ty 是遍历传入的 arena 存活句柄。
  unsafe { (*(owner as *mut ReplaceGenerics)).is_dirty_type_id(ty) }
}

fn replace_generics_is_dirty_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 同上——owner 回转合法且存活；is_dirty_type_pack_id 只读判定，tp 为
  // 遍历传入的 arena 存活 TypePackId，借用止于回调返回。
  unsafe { (*(owner as *mut ReplaceGenerics)).is_dirty_type_pack_id(tp) }
}

fn replace_generics_clean_ty(owner: *mut (), ty: TypeId) -> TypeId {
  // Safety: owner 为遍历期被调用栈独占存活的 ReplaceGenerics；clean_type_id 以
  // &mut self 就地改写其 Substitution 状态，借用覆盖本次调用且无其它持有者，
  // ty 存活。
  unsafe { (*(owner as *mut ReplaceGenerics)).clean_type_id(ty) }
}

fn replace_generics_clean_tp(owner: *mut (), tp: TypePackId) -> TypePackId {
  // Safety: 同 clean_ty——owner 存活非空，clean_type_pack_id 的 &mut 再借用只
  // 活过本次调用；tp 为遍历传入的 arena 存活句柄，无并存别名。
  unsafe { (*(owner as *mut ReplaceGenerics)).clean_type_pack_id(tp) }
}

fn replace_generics_found_dirty_ty(owner: *mut (), ty: TypeId) {
  // Safety: owner 存活；经 `.base` 短借其 Substitution 记录 dirty TypeId，
  // found_dirty_type_id 为 unsafe fn，其「ty 存活、Substitution 处于遍历中」
  // 前置由遍历实参与调用栈保证，借用止于回调。
  unsafe {
    (*(owner as *mut ReplaceGenerics))
      .base
      .found_dirty_type_id(ty)
  }
}

fn replace_generics_found_dirty_tp(owner: *mut (), tp: TypePackId) {
  // Safety: 同 found_dirty_ty——owner 为遍历期独占存活句柄，tp 存活；
  // found_dirty_type_pack_id 为 unsafe fn，其遍历期 &mut 前置成立，经 `.base`
  // 的可变借用仅在本回调窗口内，无并发持有者。
  unsafe {
    (*(owner as *mut ReplaceGenerics))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn replace_generics_ignore_children_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 回转合法（唯一装配点 install_substitution_vtable）；
  // ignore_children 取 &self 只读判定，ty 为遍历传入的 arena 存活句柄。
  unsafe { (*(owner as *mut ReplaceGenerics)).ignore_children(ty) }
}

fn replace_generics_ignore_children_tp(_owner: *mut (), _tp: TypePackId) -> bool {
  false
}

impl ReplaceGenerics {
  /// C++ `ReplaceGenerics(const TxnLog* log, TypeArena* arena, NotNull<BuiltinTypes> builtinTypes,
  /// TypeLevel level, Scope* scope, const std::vector<TypeId>& generics,
  /// const std::vector<TypePackId>& genericPacks) : Substitution(log, arena), ...`.
  pub fn replace_generics_new(
    log: *const TxnLog,
    arena: Option<Handle<TypeArena>>,
    builtin_types: Handle<BuiltinTypes>,
    level: TypeLevel,
    scope: *mut Scope,
    generics: Vec<TypeId>,
    generic_packs: Vec<TypePackId>,
  ) -> Self {
    ReplaceGenerics {
      base: Substitution::substitution_new(log, arena),
      builtin_types,
      level,
      scope,
      generics,
      generic_packs,
    }
  }

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut ReplaceGenerics as *mut ();
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(replace_generics_is_dirty_ty),
      is_dirty_tp: Some(replace_generics_is_dirty_tp),
      clean_ty: Some(replace_generics_clean_ty),
      clean_tp: Some(replace_generics_clean_tp),
      found_dirty_ty: Some(replace_generics_found_dirty_ty),
      found_dirty_tp: Some(replace_generics_found_dirty_tp),
      ignore_children_ty: Some(replace_generics_ignore_children_ty),
      ignore_children_tp: Some(replace_generics_ignore_children_tp),
      ignore_children_visit_ty: Some(replace_generics_ignore_children_ty),
      ignore_children_visit_tp: Some(replace_generics_ignore_children_tp),
    };
  }

  substitution_entry!(id, pack);
}
