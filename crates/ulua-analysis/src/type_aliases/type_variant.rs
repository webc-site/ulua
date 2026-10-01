use crate::{
  macros::variant_member,
  records::{
    any_type::AnyType, blocked_type::BlockedType, extern_type::ExternType, free_type::FreeType,
    function_type::FunctionType, generic_type::GenericType, intersection_type::IntersectionType,
    lazy_type::LazyType, metatable_type::MetatableType, negation_type::NegationType,
    never_type::NeverType, no_refine_type::NoRefineType,
    pending_expansion_type::PendingExpansionType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType, unifiable::Error, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{bound_type::BoundType, type_id::TypeId},
};
#[derive(Debug, Clone)]
pub enum TypeVariant {
  Bound(TypeId),
  Error(Error<TypeId>),
  Free(FreeType),
  Generic(GenericType),
  Primitive(PrimitiveType),
  Singleton(SingletonType),
  Blocked(BlockedType),
  PendingExpansion(PendingExpansionType),
  Function(FunctionType),
  Table(TableType),
  Metatable(MetatableType),
  Extern(ExternType),
  Any(AnyType),
  Union(UnionType),
  Intersection(IntersectionType),
  Lazy(LazyType),
  Unknown(UnknownType),
  Never(NeverType),
  Negation(NegationType),
  NoRefine(NoRefineType),
  TypeFunctionInstance(TypeFunctionInstanceType),
}

impl TypeVariant {
  pub fn index(&self) -> i32 {
    match self {
      TypeVariant::Bound(_) => 0,
      TypeVariant::Error(_) => 1,
      TypeVariant::Free(_) => 2,
      TypeVariant::Generic(_) => 3,
      TypeVariant::Primitive(_) => 4,
      TypeVariant::Singleton(_) => 5,
      TypeVariant::Blocked(_) => 6,
      TypeVariant::PendingExpansion(_) => 7,
      TypeVariant::Function(_) => 8,
      TypeVariant::Table(_) => 9,
      TypeVariant::Metatable(_) => 10,
      TypeVariant::Extern(_) => 11,
      TypeVariant::Any(_) => 12,
      TypeVariant::Union(_) => 13,
      TypeVariant::Intersection(_) => 14,
      TypeVariant::Lazy(_) => 15,
      TypeVariant::Unknown(_) => 16,
      TypeVariant::Never(_) => 17,
      TypeVariant::Negation(_) => 18,
      TypeVariant::NoRefine(_) => 19,
      TypeVariant::TypeFunctionInstance(_) => 20,
    }
  }
}

// `Bound` 由下方 `BoundType` 的手工实现承接（槽位存裸 `TypeId`），不在表内。
variant_member! {
  /// The Rust shape of C++ `Luau::get_if<T>(&TypeVariant)`: one trait impl per
  /// alternative lets `get::<T>(ty)` keep the C++ call shape at every call site.
  enum TypeVariantMember: TypeVariant {
    Error => Error<TypeId>,
    Free => FreeType,
    Generic => GenericType,
    Primitive => PrimitiveType,
    Singleton => SingletonType,
    Blocked => BlockedType,
    PendingExpansion => PendingExpansionType,
    Function => FunctionType,
    Table => TableType,
    Metatable => MetatableType,
    Extern => ExternType,
    Any => AnyType,
    Union => UnionType,
    Intersection => IntersectionType,
    Lazy => LazyType,
    Unknown => UnknownType,
    Never => NeverType,
    Negation => NegationType,
    NoRefine => NoRefineType,
    TypeFunctionInstance => TypeFunctionInstanceType,
  }
}

/// BoundType = Bound<TypeId>; the Bound alternative stores the bare TypeId.
impl TypeVariantMember for BoundType {
  fn get_if(v: &TypeVariant) -> Option<&Self> {
    match v {
      TypeVariant::Bound(inner) => {
        // Bound<TypeId> is repr-compatible with its single TypeId field.
        Some(unsafe { &*(inner as *const TypeId as *const Self) })
      }
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeVariant) -> Option<&mut Self> {
    match v {
      TypeVariant::Bound(inner) => Some(unsafe { &mut *(inner as *mut TypeId as *mut Self) }),
      _ => None,
    }
  }
}
