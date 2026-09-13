use core::ffi::c_void;

use crate::{
  records::{
    builtin_types::BuiltinTypes, substitution::Substitution, tarjan::SubstitutionVtable,
    txn_log::TxnLog, type_arena::TypeArena, widen::Widen,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn widen_is_dirty_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut Widen)).is_dirty_type_id(ty) }
}

fn widen_is_dirty_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut Widen)).is_dirty_type_pack_id(tp) }
}

fn widen_clean_ty(owner: *mut c_void, ty: TypeId) -> TypeId {
  unsafe { (*(owner as *mut Widen)).clean_type_id(ty) }
}

fn widen_clean_tp(owner: *mut c_void, tp: TypePackId) -> TypePackId {
  unsafe { (*(owner as *mut Widen)).clean_type_pack_id(tp) }
}

fn widen_found_dirty_ty(owner: *mut c_void, ty: TypeId) {
  unsafe { (*(owner as *mut Widen)).base.found_dirty_type_id(ty) }
}

fn widen_found_dirty_tp(owner: *mut c_void, tp: TypePackId) {
  unsafe { (*(owner as *mut Widen)).base.found_dirty_type_pack_id(tp) }
}

fn widen_ignore_children_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut Widen)).widen_ignore_children(ty) }
}

fn widen_ignore_children_tp(_owner: *mut c_void, _tp: TypePackId) -> bool {
  false
}

impl Widen {
  pub fn widen_widen(arena: *mut TypeArena, builtin_types: *mut BuiltinTypes) -> Self {
    Widen {
      base: Substitution::substitution_new(TxnLog::empty(), arena),
      builtin_types,
    }
  }

  pub(crate) fn install_substitution_vtable(&mut self) {
    let owner = self as *mut Widen as *mut c_void;
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(widen_is_dirty_ty),
      is_dirty_tp: Some(widen_is_dirty_tp),
      clean_ty: Some(widen_clean_ty),
      clean_tp: Some(widen_clean_tp),
      found_dirty_ty: Some(widen_found_dirty_ty),
      found_dirty_tp: Some(widen_found_dirty_tp),
      ignore_children_ty: Some(widen_ignore_children_ty),
      ignore_children_tp: Some(widen_ignore_children_tp),
      ignore_children_visit_ty: Some(widen_ignore_children_ty),
      ignore_children_visit_tp: Some(widen_ignore_children_tp),
    };
  }
}
