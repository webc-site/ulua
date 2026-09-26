//! Source: `Analysis/include/Luau/VisitType.h:70-215` (hand-ported)
//!
//! C++ `template<typename S> struct GenericTypeVisitor` with ~30 virtual
//! `visit(...)` overloads and two `traverse(...)` drivers. Rust shape (the
//! AstVisitor precedent from the encoder port):
//!   methods (pinned overload names: `visit_type_id_free_type`, ...);
//!   subclasses embed the base struct and override what they need.
//!   provided trait methods whose bodies live in the traverse node files.
//!   (std set forgets on unsee; DenseHashSet — visit-once — does not).

use alloc::string::String;

use ulua_common::records::dense_hash_set::DenseHashSet;

/// `visit_detail::hasSeen/unsee` (VisitType.h:36-66) — the seen-set policy
/// that C++ selects by overload on the set type.
use crate::functions::has_seen_visit_type;
use crate::{
  functions::{
    has_seen_visit_type::has_seen_dense_hash_set_void_void, unsee_visit_type,
    unsee_visit_type::unsee_dense_hash_set_void_void,
  },
  macros::visit_type_delegators,
  methods::generic_type_visitor_traverse_visit_type,
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
    bound_type::BoundType, bound_type_pack::BoundTypePack, collections::HashSet,
    error_type::ErrorType, error_type_pack::ErrorTypePack, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
pub trait VisitSeen {
  fn has_seen(&mut self, tv: *const ()) -> bool;
  fn unsee(&mut self, tv: *const ());
}

impl VisitSeen for HashSet<*mut ()> {
  fn has_seen(&mut self, tv: *const ()) -> bool {
    has_seen_visit_type::has_seen(self, tv)
  }
  fn unsee(&mut self, tv: *const ()) {
    unsee_visit_type::unsee(self, tv)
  }
}

impl VisitSeen for DenseHashSet<*mut ()> {
  fn has_seen(&mut self, tv: *const ()) -> bool {
    has_seen_dense_hash_set_void_void(self, tv)
  }
  fn unsee(&mut self, tv: *const ()) {
    unsee_dense_hash_set_void_void(self, tv)
  }
}

/// Base state of C++ `GenericTypeVisitor<S>` (VisitType.h:70-90).
#[derive(Debug, Clone)]
pub struct GenericTypeVisitor<S = HashSet<*mut ()>> {
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
  fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    true
  }

  // 28 个按变体重载的默认方法由共享宏生成，逐条转发到上面的裸 visit，
  // 与 C++ `GenericTypeVisitor` 的默认实现等价。
  visit_type_delegators!();

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
    generic_type_visitor_traverse_visit_type::traverse_type_pack_id(self, tp)
  }
}
