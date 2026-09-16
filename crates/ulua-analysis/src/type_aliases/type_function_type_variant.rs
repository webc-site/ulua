//! Source: `Analysis/include/Luau/TypeFunctionRuntime.h:248-261` (hand-ported)
use crate::records::{
  type_function_any_type::TypeFunctionAnyType, type_function_extern_type::TypeFunctionExternType,
  type_function_function_type::TypeFunctionFunctionType,
  type_function_generic_type::TypeFunctionGenericType,
  type_function_intersection_type::TypeFunctionIntersectionType,
  type_function_negation_type::TypeFunctionNegationType,
  type_function_never_type::TypeFunctionNeverType,
  type_function_primitive_type::TypeFunctionPrimitiveType,
  type_function_singleton_type::TypeFunctionSingletonType,
  type_function_table_type::TypeFunctionTableType, type_function_union_type::TypeFunctionUnionType,
  type_function_unknown_type::TypeFunctionUnknownType,
};

// 12 members exceed the fixed-arity Variant7 family, so this is a custom
// enum (the TypeVariant precedent). Member ORDER preserves the C++ Variant
// positions so index() agrees with C++ v.index().
#[derive(Debug, Clone)]
pub enum TypeFunctionTypeVariant {
  Primitive(TypeFunctionPrimitiveType),
  Any(TypeFunctionAnyType),
  Unknown(TypeFunctionUnknownType),
  Never(TypeFunctionNeverType),
  Singleton(TypeFunctionSingletonType),
  Union(TypeFunctionUnionType),
  Intersection(TypeFunctionIntersectionType),
  Negation(TypeFunctionNegationType),
  Function(TypeFunctionFunctionType),
  Table(TypeFunctionTableType),
  Extern(TypeFunctionExternType),
  Generic(TypeFunctionGenericType),
}

impl TypeFunctionTypeVariant {
  pub fn index(&self) -> i32 {
    match self {
      TypeFunctionTypeVariant::Primitive(_) => 0,
      TypeFunctionTypeVariant::Any(_) => 1,
      TypeFunctionTypeVariant::Unknown(_) => 2,
      TypeFunctionTypeVariant::Never(_) => 3,
      TypeFunctionTypeVariant::Singleton(_) => 4,
      TypeFunctionTypeVariant::Union(_) => 5,
      TypeFunctionTypeVariant::Intersection(_) => 6,
      TypeFunctionTypeVariant::Negation(_) => 7,
      TypeFunctionTypeVariant::Function(_) => 8,
      TypeFunctionTypeVariant::Table(_) => 9,
      TypeFunctionTypeVariant::Extern(_) => 10,
      TypeFunctionTypeVariant::Generic(_) => 11,
    }
  }
}

/// `get_if<T>(&tv->type)` over the runtime type variant.
pub trait TypeFunctionTypeVariantMember: Sized {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self>;
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self>;
}

impl TypeFunctionTypeVariantMember for TypeFunctionPrimitiveType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Primitive(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Primitive(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeFunctionTypeVariantMember for TypeFunctionAnyType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Any(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Any(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeFunctionTypeVariantMember for TypeFunctionUnknownType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Unknown(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Unknown(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeFunctionTypeVariantMember for TypeFunctionNeverType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Never(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Never(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeFunctionTypeVariantMember for TypeFunctionSingletonType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Singleton(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Singleton(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeFunctionTypeVariantMember for TypeFunctionUnionType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Union(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Union(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeFunctionTypeVariantMember for TypeFunctionIntersectionType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Intersection(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Intersection(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeFunctionTypeVariantMember for TypeFunctionNegationType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Negation(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Negation(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeFunctionTypeVariantMember for TypeFunctionFunctionType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Function(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Function(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeFunctionTypeVariantMember for TypeFunctionTableType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Table(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Table(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeFunctionTypeVariantMember for TypeFunctionExternType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Extern(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Extern(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeFunctionTypeVariantMember for TypeFunctionGenericType {
  fn get_if(v: &TypeFunctionTypeVariant) -> Option<&Self> {
    match v {
      TypeFunctionTypeVariant::Generic(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeFunctionTypeVariant) -> Option<&mut Self> {
    match v {
      TypeFunctionTypeVariant::Generic(x) => Some(x),
      _ => None,
    }
  }
}
