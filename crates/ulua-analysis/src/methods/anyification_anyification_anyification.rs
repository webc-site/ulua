use core::ffi::c_void;

use crate::{
  records::{
    anyification::Anyification, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, scope::Scope, substitution::Substitution,
    tarjan::SubstitutionVtable, txn_log::TxnLog, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn anyification_is_dirty_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut Anyification)).is_dirty_type_id(ty) }
}

fn anyification_is_dirty_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut Anyification)).is_dirty_type_pack_id(tp) }
}

fn anyification_clean_ty(owner: *mut c_void, ty: TypeId) -> TypeId {
  unsafe { (*(owner as *mut Anyification)).clean_type_id(ty) }
}

fn anyification_clean_tp(owner: *mut c_void, tp: TypePackId) -> TypePackId {
  unsafe { (*(owner as *mut Anyification)).clean_type_pack_id(tp) }
}

fn anyification_found_dirty_ty(owner: *mut c_void, ty: TypeId) {
  unsafe { (*(owner as *mut Anyification)).base.found_dirty_type_id(ty) }
}

fn anyification_found_dirty_tp(owner: *mut c_void, tp: TypePackId) {
  unsafe {
    (*(owner as *mut Anyification))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn anyification_ignore_children_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut Anyification)).ignore_children_type_id(ty) }
}

fn anyification_ignore_children_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut Anyification)).ignore_children_type_pack_id(tp) }
}

impl Anyification {
  pub fn anyification_type_arena_not_null_scope_not_null_builtin_types_internal_error_reporter_type_id_type_pack_id(
    arena: *mut TypeArena,
    scope: *mut Scope,
    builtin_types: *mut BuiltinTypes,
    ice_handler: *mut InternalErrorReporter,
    any_type: TypeId,
    any_type_pack: TypePackId,
  ) -> Self {
    Anyification {
      base: Substitution::substitution_new(TxnLog::empty(), arena),
      scope,
      builtin_types,
      ice_handler,
      any_type,
      any_type_pack,
      normalization_too_complex: false,
    }
  }

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut Anyification as *mut c_void;
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(anyification_is_dirty_ty),
      is_dirty_tp: Some(anyification_is_dirty_tp),
      clean_ty: Some(anyification_clean_ty),
      clean_tp: Some(anyification_clean_tp),
      found_dirty_ty: Some(anyification_found_dirty_ty),
      found_dirty_tp: Some(anyification_found_dirty_tp),
      ignore_children_ty: Some(anyification_ignore_children_ty),
      ignore_children_tp: Some(anyification_ignore_children_tp),
      ignore_children_visit_ty: Some(anyification_ignore_children_ty),
      ignore_children_visit_tp: Some(anyification_ignore_children_tp),
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
