//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/VisitType.h:70:generic_type_visitor`
//! Source: `Analysis/include/Luau/VisitType.h:70-215` (hand-ported)
//!
//! C++ `template<typename S> struct GenericTypeVisitor` with ~30 virtual
//! `visit(...)` overloads and two `traverse(...)` drivers. Rust shape (the
//! AstVisitor precedent from the encoder port):
//! - `GenericTypeVisitor<S>` holds the base state (visitorName/seen/...).
//! - `GenericTypeVisitorTrait` carries the virtual surface as default
//!   methods (pinned overload names: `visit_type_id_free_type`, ...);
//!   subclasses embed the base struct and override what they need.
//! - `traverse_type_id`/`traverse_type_pack_id` (VisitType.h:217/444) are
//!   provided trait methods whose bodies live in the traverse node files.
//! - `VisitSeen` mirrors the `visit_detail::hasSeen/unsee` overload pair
//!   (std set forgets on unsee; DenseHashSet — visit-once — does not).

use alloc::string::String;
use core::ffi::c_void;
use std::collections::HashSet;

use ulua_common::records::dense_hash_set::DenseHashSet;

/// `visit_detail::hasSeen/unsee` (VisitType.h:36-66) — the seen-set policy
/// that C++ selects by overload on the set type.
use crate::functions::has_seen_visit_type;
use crate::{
  functions::{
    has_seen_visit_type_alt_b::has_seen_dense_hash_set_void_void, unsee_visit_type,
    unsee_visit_type_alt_b::unsee_dense_hash_set_void_void,
  },
  methods::{
    generic_type_visitor_traverse_visit_type, generic_type_visitor_traverse_visit_type_alt_b,
  },
  records::{
    any_type::AnyType, blocked_type::BlockedType, blocked_type_pack::BlockedTypePack,
    extern_type::ExternType, free_type::FreeType, free_type_pack::FreeTypePack,
    function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, never_type::NeverType, no_refine_type::NoRefineType,
    pending_expansion_type::PendingExpansionType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    union_type::UnionType, unknown_type::UnknownType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, error_type::ErrorType,
    error_type_pack::ErrorTypePack, type_id::TypeId, type_pack_id::TypePackId,
  },
};
pub trait VisitSeen {
  fn has_seen(&mut self, tv: *const c_void) -> bool;
  fn unsee(&mut self, tv: *const c_void);
}

impl VisitSeen for HashSet<*mut c_void> {
  fn has_seen(&mut self, tv: *const c_void) -> bool {
    has_seen_visit_type::has_seen(self, tv)
  }
  fn unsee(&mut self, tv: *const c_void) {
    unsee_visit_type::unsee(self, tv)
  }
}

impl VisitSeen for DenseHashSet<*mut c_void> {
  fn has_seen(&mut self, tv: *const c_void) -> bool {
    has_seen_dense_hash_set_void_void(self, tv)
  }
  fn unsee(&mut self, tv: *const c_void) {
    unsee_dense_hash_set_void_void(self, tv)
  }
}

/// Base state of C++ `GenericTypeVisitor<S>` (VisitType.h:70-90).
#[derive(Debug, Clone)]
pub struct GenericTypeVisitor<S = HashSet<*mut c_void>> {
  pub visitor_name: String,
  pub seen: S,
  pub skip_bound_types: bool,
  pub recursion_counter: i32,
  pub type_function_depth: i32,
}

/// The virtual surface of `GenericTypeVisitor` (VisitType.h:92-215). Default
/// bodies are the C++ defaults: per-member overloads delegate to the bare
/// `visit(TypeId)`/`visit(TypePackId)`, which default to `true`.
pub trait GenericTypeVisitorTrait {
  type Seen: VisitSeen;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen>;

  fn cycle_type_id(&mut self, _ty: TypeId) {}
  fn cycle_type_pack_id(&mut self, _tp: TypePackId) {}

  fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    true
  }
  fn visit_type_id_bound_type(&mut self, ty: TypeId, _btv: &BoundType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_free_type(&mut self, ty: TypeId, _ftv: &FreeType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_generic_type(&mut self, ty: TypeId, _gtv: &GenericType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_error_type(&mut self, ty: TypeId, _etv: &ErrorType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_primitive_type(&mut self, ty: TypeId, _ptv: &PrimitiveType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_function_type(&mut self, ty: TypeId, _ftv: &FunctionType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_table_type(&mut self, ty: TypeId, _ttv: &TableType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_metatable_type(&mut self, ty: TypeId, _mtv: &MetatableType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_extern_type(&mut self, ty: TypeId, _etv: &ExternType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_any_type(&mut self, ty: TypeId, _atv: &AnyType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_no_refine_type(&mut self, ty: TypeId, _nrt: &NoRefineType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_unknown_type(&mut self, ty: TypeId, _utv: &UnknownType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_never_type(&mut self, ty: TypeId, _ntv: &NeverType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_union_type(&mut self, ty: TypeId, _utv: &UnionType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_intersection_type(&mut self, ty: TypeId, _itv: &IntersectionType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_blocked_type(&mut self, ty: TypeId, _btv: &BlockedType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_singleton_type(&mut self, ty: TypeId, _stv: &SingletonType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_negation_type(&mut self, ty: TypeId, _ntv: &NegationType) -> bool {
    self.visit_type_id(ty)
  }
  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    _tfit: &TypeFunctionInstanceType,
  ) -> bool {
    self.visit_type_id(ty)
  }

  fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    true
  }
  fn visit_type_pack_id_bound_type_pack(&mut self, tp: TypePackId, _btp: &BoundTypePack) -> bool {
    self.visit_type_pack_id(tp)
  }
  fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, _ftp: &FreeTypePack) -> bool {
    self.visit_type_pack_id(tp)
  }
  fn visit_type_pack_id_generic_type_pack(
    &mut self,
    tp: TypePackId,
    _gtp: &GenericTypePack,
  ) -> bool {
    self.visit_type_pack_id(tp)
  }
  fn visit_type_pack_id_error_type_pack(&mut self, tp: TypePackId, _etp: &ErrorTypePack) -> bool {
    self.visit_type_pack_id(tp)
  }
  fn visit_type_pack_id_type_pack(&mut self, tp: TypePackId, _pack: &TypePack) -> bool {
    self.visit_type_pack_id(tp)
  }
  fn visit_type_pack_id_variadic_type_pack(
    &mut self,
    tp: TypePackId,
    _vtp: &VariadicTypePack,
  ) -> bool {
    self.visit_type_pack_id(tp)
  }
  fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    tp: TypePackId,
    _btp: &BlockedTypePack,
  ) -> bool {
    self.visit_type_pack_id(tp)
  }
  fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    _tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    self.visit_type_pack_id(tp)
  }

  /// C++ `void traverse(TypeId ty)` (VisitType.h:217). Body in the traverse
  /// node file.
  fn traverse_type_id(&mut self, ty: TypeId)
  where
    Self: Sized,
  {
    generic_type_visitor_traverse_visit_type::traverse_type_id(self, ty)
  }

  /// C++ `void traverse(TypePackId tp)` (VisitType.h:444).
  fn traverse_type_pack_id(&mut self, tp: TypePackId)
  where
    Self: Sized,
  {
    generic_type_visitor_traverse_visit_type_alt_b::traverse_type_pack_id(self, tp)
  }
}
