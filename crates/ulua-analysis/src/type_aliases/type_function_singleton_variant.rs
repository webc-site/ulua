use ulua_common::records::variant::Variant2;

use crate::records::{
  type_function_boolean_singleton::TypeFunctionBooleanSingleton,
  type_function_string_singleton::TypeFunctionStringSingleton,
};

pub type TypeFunctionSingletonVariant =
  Variant2<TypeFunctionBooleanSingleton, TypeFunctionStringSingleton>;

/// `get_if<T>(&tv->variant)` over the runtime singleton variant
/// (TypeFunctionRuntime.h:82/88).
pub trait TypeFunctionSingletonVariantMember: Sized {
  fn get_if(v: &TypeFunctionSingletonVariant) -> Option<&Self>;
  fn get_if_mut(v: &mut TypeFunctionSingletonVariant) -> Option<&mut Self>;
}

impl TypeFunctionSingletonVariantMember for TypeFunctionBooleanSingleton {
  fn get_if(v: &TypeFunctionSingletonVariant) -> Option<&Self> {
    v.get_if_0()
  }
  fn get_if_mut(v: &mut TypeFunctionSingletonVariant) -> Option<&mut Self> {
    v.get_if_0_mut()
  }
}

impl TypeFunctionSingletonVariantMember for TypeFunctionStringSingleton {
  fn get_if(v: &TypeFunctionSingletonVariant) -> Option<&Self> {
    v.get_if_1()
  }
  fn get_if_mut(v: &mut TypeFunctionSingletonVariant) -> Option<&mut Self> {
    v.get_if_1_mut()
  }
}
