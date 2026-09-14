//! `UnscopedGenericFinder` visitor overrides (TypeFunction.cpp:148-197).
//!
//! C++ `struct UnscopedGenericFinder : TypeOnceVisitor` with `visit` overrides.
//! The virtual surface is rendered here as the `GenericTypeVisitorTrait` impl
//! (the precedent is `FindCyclicTypes`/`InternalTypeFunctionFinder`), so that
//! `traverse` dispatches into these overrides. The peer `*_visit_*` node files
//! for the individual overloads (148/154/160/168) are comment-stubs that point
//! here.
//!
//! Covers:
//! - `visit(TypeId)` (148-152)
//! - `visit(TypePackId)` (154-158)
//! - `visit(TypeId, const GenericType&)` (160-166)
//! - `visit(TypePackId, const GenericTypePack&)` (168-174)
//! - `visit(TypeId, const FunctionType&)` (176-191)
//! - `visit(TypeId, const ExternType&)` (193-196)

use core::ffi::c_void;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    extern_type::ExternType,
    function_type::FunctionType,
    generic_type::GenericType,
    generic_type_pack::GenericTypePack,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    unscoped_generic_finder::UnscopedGenericFinder,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl GenericTypeVisitorTrait for UnscopedGenericFinder {
  type Seen = DenseHashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  /// TypeFunction.cpp:148 — stop the traversal once an unscoped generic is found.
  fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    !self.found_unscoped
  }

  /// TypeFunction.cpp:154 — stop the traversal once an unscoped generic is found.
  fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    !self.found_unscoped
  }

  /// TypeFunction.cpp:160
  fn visit_type_id_generic_type(&mut self, ty: TypeId, _gtv: &GenericType) -> bool {
    if !self.scope_gen_tys.contains(&ty) {
      self.found_unscoped = true;
    }

    false
  }

  /// TypeFunction.cpp:168
  fn visit_type_pack_id_generic_type_pack(
    &mut self,
    tp: TypePackId,
    _gtp: &GenericTypePack,
  ) -> bool {
    if !self.scope_gen_tps.contains(&tp) {
      self.found_unscoped = true;
    }

    false
  }

  /// TypeFunction.cpp:176
  fn visit_type_id_function_type(&mut self, _ty: TypeId, ftv: &FunctionType) -> bool {
    let start_ty_count = self.scope_gen_tys.len();
    let start_tp_count = self.scope_gen_tps.len();

    self.scope_gen_tys.extend_from_slice(&ftv.generics);
    self.scope_gen_tps.extend_from_slice(&ftv.generic_packs);

    self.traverse_type_pack_id(ftv.arg_types);
    self.traverse_type_pack_id(ftv.ret_types);

    self.scope_gen_tys.truncate(start_ty_count);
    self.scope_gen_tps.truncate(start_tp_count);

    false
  }

  /// TypeFunction.cpp:193
  fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }
}
