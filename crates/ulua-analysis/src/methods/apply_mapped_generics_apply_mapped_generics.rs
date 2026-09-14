use core::ffi::c_void;

use crate::{
  records::{
    apply_mapped_generics::ApplyMappedGenerics, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, subtyping_environment::SubtypingEnvironment,
    tarjan::SubstitutionVtable, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ApplyMappedGenerics {
  pub fn apply_mapped_generics(
    &mut self,
    builtin_types: *mut BuiltinTypes,
    arena: *mut TypeArena,
    env: &mut SubtypingEnvironment,
    ice_reporter: *mut InternalErrorReporter,
  ) {
    self.builtin_types = builtin_types;
    self.arena = arena;
    self.env = env as *mut _;
    self.ice_reporter = ice_reporter;
  }
}

// ---------------------------------------------------------------------------
// Substitution virtual-override dispatch for `ApplyMappedGenerics`.
//
// C++ `ApplyMappedGenerics` overrides `isDirty` / `clean` / `ignoreChildren`
// (it does NOT override `ignoreChildrenVisit`, so that defaults to
// `ignoreChildren`). The inherited `substitute` traversal dispatches into these
// at runtime. Each thunk casts the type-erased `owner` data pointer back to the
// concrete `*mut ApplyMappedGenerics` it was installed for and calls the inherent
// override. `found_dirty` is `Substitution`'s own (non-overridden) method, so its
// thunk forwards to `self.base.found_dirty_*`.
// ---------------------------------------------------------------------------

fn amg_is_dirty_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut ApplyMappedGenerics)).is_dirty_type_id(ty) }
}

fn amg_is_dirty_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut ApplyMappedGenerics)).is_dirty_type_pack_id(tp) }
}

fn amg_clean_ty(owner: *mut c_void, ty: TypeId) -> TypeId {
  unsafe { (*(owner as *mut ApplyMappedGenerics)).clean_type_id(ty) }
}

fn amg_clean_tp(owner: *mut c_void, tp: TypePackId) -> TypePackId {
  unsafe { (*(owner as *mut ApplyMappedGenerics)).clean_type_pack_id(tp) }
}

fn amg_found_dirty_ty(owner: *mut c_void, ty: TypeId) {
  unsafe {
    (*(owner as *mut ApplyMappedGenerics))
      .base
      .found_dirty_type_id(ty)
  }
}

fn amg_found_dirty_tp(owner: *mut c_void, tp: TypePackId) {
  unsafe {
    (*(owner as *mut ApplyMappedGenerics))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn amg_ignore_children_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut ApplyMappedGenerics)).ignore_children_type_id(ty) }
}

fn amg_ignore_children_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut ApplyMappedGenerics)).ignore_children_type_pack_id(tp) }
}

// AMG does not override ignoreChildrenVisit; the base default forwards to ignoreChildren.
fn amg_ignore_children_visit_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut ApplyMappedGenerics)).ignore_children_type_id(ty) }
}

fn amg_ignore_children_visit_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut ApplyMappedGenerics)).ignore_children_type_pack_id(tp) }
}

impl ApplyMappedGenerics {
  /// Point the embedded `Substitution`'s dispatch table at this object and its
  /// overrides. Must run with `self` at its final address (i.e. from a method
  /// called on a settled `&mut self`), so the `owner` pointer stays valid for
  /// the duration of the `substitute` traversal.
  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut ApplyMappedGenerics as *mut c_void;
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(amg_is_dirty_ty),
      is_dirty_tp: Some(amg_is_dirty_tp),
      clean_ty: Some(amg_clean_ty),
      clean_tp: Some(amg_clean_tp),
      found_dirty_ty: Some(amg_found_dirty_ty),
      found_dirty_tp: Some(amg_found_dirty_tp),
      ignore_children_ty: Some(amg_ignore_children_ty),
      ignore_children_tp: Some(amg_ignore_children_tp),
      ignore_children_visit_ty: Some(amg_ignore_children_visit_ty),
      ignore_children_visit_tp: Some(amg_ignore_children_visit_tp),
    };
  }

  /// Inherited `Substitution::substitute(TypeId)` with override dispatch wired.
  pub fn substitute_type_id(&mut self, ty: TypeId) -> Option<TypeId> {
    self.install_substitution_vtable();
    self.base.substitute_type_id(ty)
  }

  /// Inherited `Substitution::substitute(TypePackId)` with override dispatch wired.
  pub fn substitute_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
    self.install_substitution_vtable();
    self.base.substitute_type_pack_id(tp)
  }
}
