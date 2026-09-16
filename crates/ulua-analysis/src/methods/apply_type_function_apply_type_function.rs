use core::{ffi::c_void, ptr::null};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    apply_type_function::ApplyTypeFunction, substitution::Substitution, tarjan::SubstitutionVtable,
    txn_log::TxnLog, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
fn apply_type_function_is_dirty_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut ApplyTypeFunction)).is_dirty_type_id(ty) }
}

fn apply_type_function_is_dirty_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut ApplyTypeFunction)).is_dirty_type_pack_id(tp) }
}

fn apply_type_function_clean_ty(owner: *mut c_void, ty: TypeId) -> TypeId {
  unsafe { (*(owner as *mut ApplyTypeFunction)).clean_type_id(ty) }
}

fn apply_type_function_clean_tp(owner: *mut c_void, tp: TypePackId) -> TypePackId {
  unsafe { (*(owner as *mut ApplyTypeFunction)).clean_type_pack_id(tp) }
}

fn apply_type_function_found_dirty_ty(owner: *mut c_void, ty: TypeId) {
  unsafe {
    (*(owner as *mut ApplyTypeFunction))
      .base
      .found_dirty_type_id(ty)
  }
}

fn apply_type_function_found_dirty_tp(owner: *mut c_void, tp: TypePackId) {
  unsafe {
    (*(owner as *mut ApplyTypeFunction))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn apply_type_function_ignore_children_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut ApplyTypeFunction)).ignore_children_type_id(ty) }
}

fn apply_type_function_ignore_children_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut ApplyTypeFunction)).ignore_children_type_pack_id(tp) }
}

impl ApplyTypeFunction {
  pub fn new(arena: *mut TypeArena) -> Self {
    Self {
      base: Substitution::substitution_new(TxnLog::empty(), arena),
      encountered_forwarded_type: false,
      type_arguments: DenseHashMap::new(null()),
      type_pack_arguments: DenseHashMap::new(null()),
    }
  }

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut ApplyTypeFunction as *mut c_void;
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(apply_type_function_is_dirty_ty),
      is_dirty_tp: Some(apply_type_function_is_dirty_tp),
      clean_ty: Some(apply_type_function_clean_ty),
      clean_tp: Some(apply_type_function_clean_tp),
      found_dirty_ty: Some(apply_type_function_found_dirty_ty),
      found_dirty_tp: Some(apply_type_function_found_dirty_tp),
      ignore_children_ty: Some(apply_type_function_ignore_children_ty),
      ignore_children_tp: Some(apply_type_function_ignore_children_tp),
      ignore_children_visit_ty: Some(apply_type_function_ignore_children_ty),
      ignore_children_visit_tp: Some(apply_type_function_ignore_children_tp),
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
