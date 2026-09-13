//! C++ `RefineTypeScrubber::RefineTypeScrubber(NotNull<TypeFunctionContext> ctx,
//! TypeId needle)` (BuiltinTypeFunctions.cpp:1083-1088). Base-inits the
//! `Substitution` with `ctx->arena`, then stores `ctx` and `needle`.
use core::{ffi::c_void, ptr::NonNull};

use crate::{
  records::{
    refine_type_scrubber::RefineTypeScrubber, substitution::Substitution,
    tarjan::SubstitutionVtable, txn_log::TxnLog, type_function_context::TypeFunctionContext,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn refine_type_scrubber_is_dirty_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut RefineTypeScrubber)).is_dirty_type_id(ty) }
}

fn refine_type_scrubber_is_dirty_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut RefineTypeScrubber)).is_dirty_type_pack_id(tp) }
}

fn refine_type_scrubber_clean_ty(owner: *mut c_void, ty: TypeId) -> TypeId {
  unsafe { (*(owner as *mut RefineTypeScrubber)).clean_type_id(ty) }
}

fn refine_type_scrubber_clean_tp(owner: *mut c_void, tp: TypePackId) -> TypePackId {
  unsafe { (*(owner as *mut RefineTypeScrubber)).clean_type_pack_id(tp) }
}

fn refine_type_scrubber_found_dirty_ty(owner: *mut c_void, ty: TypeId) {
  unsafe {
    (*(owner as *mut RefineTypeScrubber))
      .base
      .found_dirty_type_id(ty)
  }
}

fn refine_type_scrubber_found_dirty_tp(owner: *mut c_void, tp: TypePackId) {
  unsafe {
    (*(owner as *mut RefineTypeScrubber))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn refine_type_scrubber_ignore_children_ty(owner: *mut c_void, ty: TypeId) -> bool {
  unsafe { (*(owner as *mut RefineTypeScrubber)).ignore_children_type_id(ty) }
}

fn refine_type_scrubber_ignore_children_tp(owner: *mut c_void, tp: TypePackId) -> bool {
  unsafe { (*(owner as *mut RefineTypeScrubber)).ignore_children_type_pack_id(tp) }
}

impl RefineTypeScrubber {
  /// # Safety
  /// 调用方须保证 `ctx` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn new(ctx: *mut TypeFunctionContext, needle: TypeId) -> Self {
    let ctx_ref = unsafe { &*ctx };
    let base = Substitution::substitution_new(TxnLog::empty(), ctx_ref.arena.as_ptr());
    RefineTypeScrubber {
      base,
      ctx: NonNull::new(ctx).expect("RefineTypeScrubber: null TypeFunctionContext"),
      needle,
    }
  }

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut RefineTypeScrubber as *mut c_void;
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(refine_type_scrubber_is_dirty_ty),
      is_dirty_tp: Some(refine_type_scrubber_is_dirty_tp),
      clean_ty: Some(refine_type_scrubber_clean_ty),
      clean_tp: Some(refine_type_scrubber_clean_tp),
      found_dirty_ty: Some(refine_type_scrubber_found_dirty_ty),
      found_dirty_tp: Some(refine_type_scrubber_found_dirty_tp),
      ignore_children_ty: Some(refine_type_scrubber_ignore_children_ty),
      ignore_children_tp: Some(refine_type_scrubber_ignore_children_tp),
      ignore_children_visit_ty: Some(refine_type_scrubber_ignore_children_ty),
      ignore_children_visit_tp: Some(refine_type_scrubber_ignore_children_tp),
    };
  }

  pub fn substitute_type_id(&mut self, ty: TypeId) -> Option<TypeId> {
    self.install_substitution_vtable();
    self.base.substitute_type_id(ty)
  }
}
