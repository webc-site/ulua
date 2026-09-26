//! Source: `Analysis/include/Luau/Instantiation.h` (Instantiation.h:66-73, hand-ported)
use alloc::vec::Vec;

use crate::{
  macros::substitution_entry::substitution_entry,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, instantiation::Instantiation,
    replace_generics::ReplaceGenerics, scope::Scope, substitution::Substitution,
    tarjan::SubstitutionVtable, txn_log::TxnLog, type_arena::TypeArena, type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn instantiation_is_dirty_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: `owner` 是 `install_substitution_vtable` 记录的 `self as
  // *mut Instantiation as *mut ()` 类型擦除回指——同一指针回转原类型是恒等
  // 往返，非空且对齐；vtable 仅在 `substitute_*` 的同步调用栈内被回调，此
  // Instantiation 宿主存活，回调与外层 base 访问按 C++ 虚派发语义时序串行、
  // 不同时写。
  unsafe { (*(owner as *mut Instantiation)).is_dirty_type_id(ty) }
}

fn instantiation_is_dirty_tp(_owner: *mut (), _tp: TypePackId) -> bool {
  false
}

fn instantiation_clean_ty(owner: *mut (), ty: TypeId) -> TypeId {
  // Safety: 同 `instantiation_is_dirty_ty`——owner 为 vtable 内记录的 self
  // 类型擦除回指，回调期内宿主存活；`clean_type_id` 的可变借用与 base 循环
  // 的借用时序串行（C++ 虚调用 `this` 等价），无并存可变访问。
  unsafe { (*(owner as *mut Instantiation)).clean_type_id(ty) }
}

fn instantiation_clean_tp(_owner: *mut (), tp: TypePackId) -> TypePackId {
  tp
}

fn instantiation_found_dirty_ty(owner: *mut (), ty: TypeId) {
  unsafe {
    // Safety: owner 同上为 self 恒等类型擦除回指，宿主在回调栈内存活；
    // found_dirty_type_id 只记脏标记到 base 状态，单线程串行写。
    (*(owner as *mut Instantiation))
      .base
      .found_dirty_type_id(ty)
  }
}

fn instantiation_found_dirty_tp(owner: *mut (), tp: TypePackId) {
  unsafe {
    // Safety: 同 found_dirty_ty——恒等回转的 self 回指，回调期宿主存活，
    // 对 base 的登记写入时序串行。
    (*(owner as *mut Instantiation))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn instantiation_ignore_children_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 为 vtable 记录的 self 类型擦除回指，恒等往返；回调仅
  // &self 只读判定，宿主在 `substitute_*` 调用栈内存活。
  unsafe { (*(owner as *mut Instantiation)).ignore_children(ty) }
}

fn instantiation_ignore_children_tp(_owner: *mut (), _tp: TypePackId) -> bool {
  false
}

impl Instantiation {
  /// C++ `Instantiation(const TxnLog* log, TypeArena* arena, NotNull<BuiltinTypes> builtinTypes,
  /// TypeLevel level, Scope* scope) : Substitution(log, arena), builtinTypes(builtinTypes),
  /// level(level), scope(scope), reusableReplaceGenerics(log, arena, builtinTypes, level, scope, {}, {})`.
  pub fn instantiation_new(
    log: *const TxnLog,
    arena: Option<Handle<TypeArena>>,
    builtin_types: Handle<BuiltinTypes>,
    level: TypeLevel,
    scope: *mut Scope,
  ) -> Self {
    Instantiation {
      base: Substitution::substitution_new(log, arena),
      builtin_types,
      level,
      scope,
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

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut Instantiation as *mut ();
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(instantiation_is_dirty_ty),
      is_dirty_tp: Some(instantiation_is_dirty_tp),
      clean_ty: Some(instantiation_clean_ty),
      clean_tp: Some(instantiation_clean_tp),
      found_dirty_ty: Some(instantiation_found_dirty_ty),
      found_dirty_tp: Some(instantiation_found_dirty_tp),
      ignore_children_ty: Some(instantiation_ignore_children_ty),
      ignore_children_tp: Some(instantiation_ignore_children_tp),
      ignore_children_visit_ty: Some(instantiation_ignore_children_ty),
      ignore_children_visit_tp: Some(instantiation_ignore_children_tp),
    };
  }

  substitution_entry!(id, pack);
}
