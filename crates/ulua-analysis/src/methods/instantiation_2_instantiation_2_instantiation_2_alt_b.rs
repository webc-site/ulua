use core::ffi::c_void;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    instantiation_2::Instantiation2, scope::Scope, substitution::Substitution,
    subtyping::Subtyping, tarjan::SubstitutionVtable, txn_log::TxnLog, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn inst2_is_dirty_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut Instantiation2)).is_dirty_type_id(ty) }
}

fn inst2_is_dirty_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut Instantiation2)).is_dirty_type_pack_id(tp) }
}

fn inst2_clean_ty(owner: *mut c_void, ty: TypeId) -> TypeId {
  unsafe { (*(owner as *mut Instantiation2)).clean_type_id(ty) }
}

fn inst2_clean_tp(owner: *mut c_void, tp: TypePackId) -> TypePackId {
  unsafe { (*(owner as *mut Instantiation2)).clean_type_pack_id(tp) }
}

fn inst2_found_dirty_ty(owner: *mut c_void, ty: TypeId) {
  unsafe {
    (*(owner as *mut Instantiation2))
      .base
      .found_dirty_type_id(ty)
  }
}

fn inst2_found_dirty_tp(owner: *mut c_void, tp: TypePackId) {
  unsafe {
    (*(owner as *mut Instantiation2))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn inst2_ignore_children_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut Instantiation2)).ignore_children(ty) }
}

fn inst2_ignore_children_tp(_owner: *mut c_void, _tp: TypePackId) -> bool {
  false
}

impl Instantiation2 {
  pub fn instantiation_2_type_arena_dense_hash_map_type_id_type_id_dense_hash_map_type_pack_id_type_pack_id_not_null_subtyping_not_null_scope(
    arena: *mut TypeArena,
    generic_substitutions: DenseHashMap<TypeId, TypeId>,
    generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId>,
    subtyping: *mut Subtyping,
    scope: *mut Scope,
  ) -> Self {
    Instantiation2 {
      base: Substitution::substitution_new(TxnLog::empty(), arena),
      generic_substitutions,
      generic_pack_substitutions,
      subtyping,
      scope,
    }
  }

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut Instantiation2 as *mut c_void;
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(inst2_is_dirty_ty),
      is_dirty_tp: Some(inst2_is_dirty_tp),
      clean_ty: Some(inst2_clean_ty),
      clean_tp: Some(inst2_clean_tp),
      found_dirty_ty: Some(inst2_found_dirty_ty),
      found_dirty_tp: Some(inst2_found_dirty_tp),
      ignore_children_ty: Some(inst2_ignore_children_ty),
      ignore_children_tp: Some(inst2_ignore_children_tp),
      ignore_children_visit_ty: Some(inst2_ignore_children_ty),
      ignore_children_visit_tp: Some(inst2_ignore_children_tp),
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
