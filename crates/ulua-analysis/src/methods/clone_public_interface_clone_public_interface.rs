use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    builtin_types::BuiltinTypes, clone_public_interface::ClonePublicInterface, module::Module,
    substitution::Substitution, tarjan::SubstitutionVtable, txn_log::TxnLog,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn cpi_is_dirty_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut ClonePublicInterface)).is_dirty_type_id(ty) }
}

fn cpi_is_dirty_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut ClonePublicInterface)).is_dirty_type_pack_id(tp) }
}

fn cpi_clean_ty(owner: *mut c_void, ty: TypeId) -> TypeId {
  unsafe { (*(owner as *mut ClonePublicInterface)).clean_type_id(ty) }
}

fn cpi_clean_tp(owner: *mut c_void, tp: TypePackId) -> TypePackId {
  unsafe { (*(owner as *mut ClonePublicInterface)).clean_type_pack_id(tp) }
}

fn cpi_found_dirty_ty(owner: *mut c_void, ty: TypeId) {
  unsafe {
    (*(owner as *mut ClonePublicInterface))
      .base
      .found_dirty_type_id(ty)
  }
}

fn cpi_found_dirty_tp(owner: *mut c_void, tp: TypePackId) {
  unsafe {
    (*(owner as *mut ClonePublicInterface))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn cpi_ignore_children_visit_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut ClonePublicInterface)).ignore_children_visit_type_id(ty) }
}

fn cpi_ignore_children_visit_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut ClonePublicInterface)).ignore_children_visit_type_pack_id(tp) }
}

impl ClonePublicInterface {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn new(
    _log: *const TxnLog,
    _builtin_types: *mut BuiltinTypes,
    _module: *mut Module,
    _solver_mode: SolverMode,
  ) -> Self {
    LUAU_ASSERT!(!_module.is_null());

    let arena = unsafe { &mut (*_module).interface_types };
    let base = Substitution::substitution_new(_log, arena);

    ClonePublicInterface {
      base,
      builtin_types: _builtin_types,
      module: _module,
      solver_mode: _solver_mode,
      internal_type_escaped: false,
    }
  }

  pub(crate) fn install_substitution_vtable(&mut self) {
    let owner = self as *mut ClonePublicInterface as *mut c_void;
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(cpi_is_dirty_ty),
      is_dirty_tp: Some(cpi_is_dirty_tp),
      clean_ty: Some(cpi_clean_ty),
      clean_tp: Some(cpi_clean_tp),
      found_dirty_ty: Some(cpi_found_dirty_ty),
      found_dirty_tp: Some(cpi_found_dirty_tp),
      // ClonePublicInterface overrides ignoreChildrenVisit in C++, but
      // inherits Tarjan::ignoreChildren=false. Replacement must still
      // rewrite children of clean cloned parents that were reached
      // through dirty internal children.
      ignore_children_ty: None,
      ignore_children_tp: None,
      ignore_children_visit_ty: Some(cpi_ignore_children_visit_ty),
      ignore_children_visit_tp: Some(cpi_ignore_children_visit_tp),
    };
  }
}
