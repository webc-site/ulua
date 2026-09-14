use alloc::string::String;
use core::{ptr, ptr::null_mut};

use crate::{
  records::{
    any_type::AnyType, blocked_type::BlockedType, extern_type::ExternType, free_type::FreeType,
    function_type::FunctionType, generic_type::GenericType, intersection_type::IntersectionType,
    lazy_type::LazyType, metatable_type::MetatableType, negation_type::NegationType,
    never_type::NeverType, no_refine_type::NoRefineType,
    pending_expansion_type::PendingExpansionType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, table_type::TableType, type_arena::TypeArena,
    type_function_instance_type::TypeFunctionInstanceType, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{error_type::ErrorType, type_variant::TypeVariant},
};
#[derive(Debug, Clone)]
pub struct Type {
  pub ty: TypeVariant,
  /// Kludge: A persistent Type is one that belongs to the global scope.
  /// Global type bindings are immutable but are reused many times.
  /// Persistent Types do not get cloned.
  pub persistent: bool,
  pub documentation_symbol: Option<String>,
  /// Pointer to the type arena that allocated this type.
  pub owning_arena: *mut TypeArena,
}

impl Type {
  pub(crate) fn new(ty: TypeVariant) -> Self {
    Self {
      ty,
      persistent: false,
      documentation_symbol: None,
      owning_arena: null_mut(),
    }
  }

  pub(crate) fn new_with_persistence(ty: TypeVariant, persistent: bool) -> Self {
    Self {
      ty,
      persistent,
      documentation_symbol: None,
      owning_arena: null_mut(),
    }
  }
}

/// `Type::operator==` delegates to `areEqual` (structural equality) in C++.
/// Stub — awaiting `areEqual` port. Two `Type` pointers (TypeId) are equal iff they are the
/// same address; structural equality is separate and handled via `areEqual`.
impl PartialEq for Type {
  fn eq(&self, other: &Self) -> bool {
    // Pointer identity: this matches the most common usage (TypeId == TypeId).
    ptr::eq(self, other)
  }
}

impl Eq for Type {}

/// `TypeArena::add_type<T: Into<Type>>` is designed to accept a bare type
/// variant (mirroring C++ `arena->addType(GenericType{...})`). These `From`
/// impls wrap each variant struct into a non-persistent `Type`, exactly as the
/// C++ `Type(variant)` constructor does.
macro_rules! impl_from_variant_for_type {
  ($variant:ident, $ty:path) => {
    impl From<$ty> for Type {
      fn from(v: $ty) -> Self {
        Type::new(TypeVariant::$variant(v))
      }
    }
  };
}

impl_from_variant_for_type!(Free, FreeType);
impl_from_variant_for_type!(Error, ErrorType);
impl_from_variant_for_type!(Generic, GenericType);
impl_from_variant_for_type!(Primitive, PrimitiveType);
impl_from_variant_for_type!(Singleton, SingletonType);
impl_from_variant_for_type!(Blocked, BlockedType);
impl_from_variant_for_type!(PendingExpansion, PendingExpansionType);
impl_from_variant_for_type!(Function, FunctionType);
impl_from_variant_for_type!(Table, TableType);
impl_from_variant_for_type!(Metatable, MetatableType);
impl_from_variant_for_type!(Extern, ExternType);
impl_from_variant_for_type!(Any, AnyType);
impl_from_variant_for_type!(Union, UnionType);
impl_from_variant_for_type!(Intersection, IntersectionType);
impl_from_variant_for_type!(Lazy, LazyType);
impl_from_variant_for_type!(Unknown, UnknownType);
impl_from_variant_for_type!(Never, NeverType);
impl_from_variant_for_type!(Negation, NegationType);
impl_from_variant_for_type!(NoRefine, NoRefineType);
impl_from_variant_for_type!(TypeFunctionInstance, TypeFunctionInstanceType);
