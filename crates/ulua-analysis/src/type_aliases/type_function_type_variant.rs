//! Source: `Analysis/include/Luau/TypeFunctionRuntime.h:248-261` (hand-ported)
use crate::{
  macros::variant_enum,
  records::{
    type_function_any_type::TypeFunctionAnyType, type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
  },
};

// 12 members exceed the fixed-arity Variant7 family, so this is a custom
// enum (the TypeVariant precedent). Member ORDER preserves the C++ Variant
// positions so index() agrees with C++ v.index().
variant_enum! {
  #[derive(Debug, Clone)]
  pub enum TypeFunctionTypeVariant {
    Primitive => TypeFunctionPrimitiveType,
    Any => TypeFunctionAnyType,
    Unknown => TypeFunctionUnknownType,
    Never => TypeFunctionNeverType,
    Singleton => TypeFunctionSingletonType,
    Union => TypeFunctionUnionType,
    Intersection => TypeFunctionIntersectionType,
    Negation => TypeFunctionNegationType,
    Function => TypeFunctionFunctionType,
    Table => TypeFunctionTableType,
    Extern => TypeFunctionExternType,
    Generic => TypeFunctionGenericType,
  }

  /// `get_if<T>(&tv->type)` over the runtime type variant.
  pub trait TypeFunctionTypeVariantMember;
}
