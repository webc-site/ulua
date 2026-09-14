use core::ffi::c_void;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  records::{
    replacer::Replacer, substitution::Substitution, tarjan::SubstitutionVtable, txn_log::TxnLog,
    type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn replacer_is_dirty_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut Replacer)).is_dirty_type_id(ty) }
}

fn replacer_is_dirty_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut Replacer)).is_dirty_type_pack_id(tp) }
}

fn replacer_clean_ty(owner: *mut c_void, ty: TypeId) -> TypeId {
  unsafe { (*(owner as *mut Replacer)).clean_type_id(ty) }
}

fn replacer_clean_tp(owner: *mut c_void, tp: TypePackId) -> TypePackId {
  unsafe { (*(owner as *mut Replacer)).clean_type_pack_id(tp) }
}

fn replacer_found_dirty_ty(owner: *mut c_void, ty: TypeId) {
  unsafe { (*(owner as *mut Replacer)).base.found_dirty_type_id(ty) }
}

fn replacer_found_dirty_tp(owner: *mut c_void, tp: TypePackId) {
  unsafe {
    (*(owner as *mut Replacer))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn replacer_ignore_children_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut Replacer)).ignore_children(ty) }
}

fn replacer_ignore_children_tp(_owner: *mut c_void, _tp: TypePackId) -> bool {
  false
}

impl Replacer {
  pub fn new(
    arena: *mut TypeArena,
    replacements: *mut DenseHashMap<TypeId, TypeId>,
    replacement_packs: *mut DenseHashMap<TypePackId, TypePackId>,
  ) -> Self {
    let this = Replacer {
      base: Substitution::substitution_new(TxnLog::empty(), arena),
      replacements,
      replacement_packs,
    };
    LUAU_ASSERT!(this.check_replacement_keys());
    this
  }

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut Replacer as *mut c_void;
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(replacer_is_dirty_ty),
      is_dirty_tp: Some(replacer_is_dirty_tp),
      clean_ty: Some(replacer_clean_ty),
      clean_tp: Some(replacer_clean_tp),
      found_dirty_ty: Some(replacer_found_dirty_ty),
      found_dirty_tp: Some(replacer_found_dirty_tp),
      ignore_children_ty: Some(replacer_ignore_children_ty),
      ignore_children_tp: Some(replacer_ignore_children_tp),
      ignore_children_visit_ty: Some(replacer_ignore_children_ty),
      ignore_children_visit_tp: Some(replacer_ignore_children_tp),
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
