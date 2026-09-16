//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Instantiation.h:21:ReplaceGenerics`
//! Source: `Analysis/include/Luau/Instantiation.h` (Instantiation.h:21-37, hand-ported)
use alloc::vec::Vec;
use core::ffi::c_void;

use crate::{
  records::{
    builtin_types::BuiltinTypes, replace_generics::ReplaceGenerics, scope::Scope,
    substitution::Substitution, tarjan::SubstitutionVtable, txn_log::TxnLog, type_arena::TypeArena,
    type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn replace_generics_is_dirty_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut ReplaceGenerics)).is_dirty_type_id(ty) }
}

fn replace_generics_is_dirty_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut ReplaceGenerics)).is_dirty_type_pack_id(tp) }
}

fn replace_generics_clean_ty(owner: *mut c_void, ty: TypeId) -> TypeId {
  unsafe { (*(owner as *mut ReplaceGenerics)).clean_type_id(ty) }
}

fn replace_generics_clean_tp(owner: *mut c_void, tp: TypePackId) -> TypePackId {
  unsafe { (*(owner as *mut ReplaceGenerics)).clean_type_pack_id(tp) }
}

fn replace_generics_found_dirty_ty(owner: *mut c_void, ty: TypeId) {
  unsafe {
    (*(owner as *mut ReplaceGenerics))
      .base
      .found_dirty_type_id(ty)
  }
}

fn replace_generics_found_dirty_tp(owner: *mut c_void, tp: TypePackId) {
  unsafe {
    (*(owner as *mut ReplaceGenerics))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn replace_generics_ignore_children_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut ReplaceGenerics)).ignore_children(ty) }
}

fn replace_generics_ignore_children_tp(_owner: *mut c_void, _tp: TypePackId) -> bool {
  false
}

impl ReplaceGenerics {
  /// C++ `ReplaceGenerics(const TxnLog* log, TypeArena* arena, NotNull<BuiltinTypes> builtinTypes,
  /// TypeLevel level, Scope* scope, const std::vector<TypeId>& generics,
  /// const std::vector<TypePackId>& genericPacks) : Substitution(log, arena), ...`.
  pub fn replace_generics_new(
    log: *const TxnLog,
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
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
    let owner = self as *mut ReplaceGenerics as *mut c_void;
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

  pub fn substitute_type_id(&mut self, ty: TypeId) -> Option<TypeId> {
    self.install_substitution_vtable();
    self.base.substitute_type_id(ty)
  }

  pub fn substitute_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
    self.install_substitution_vtable();
    self.base.substitute_type_pack_id(tp)
  }
}
