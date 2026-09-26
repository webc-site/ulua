use ulua_common::records::variant::Variant2;

use crate::{
  macros::variant_member,
  records::{
    type_function_boolean_singleton::TypeFunctionBooleanSingleton,
    type_function_string_singleton::TypeFunctionStringSingleton,
  },
};

pub type TypeFunctionSingletonVariant =
  Variant2<TypeFunctionBooleanSingleton, TypeFunctionStringSingleton>;

variant_member! {
  /// `get_if<T>(&tv->variant)` over the runtime singleton variant
  /// (TypeFunctionRuntime.h:82/88).
  position TypeFunctionSingletonVariantMember: TypeFunctionSingletonVariant {
    0 => TypeFunctionBooleanSingleton,
    1 => TypeFunctionStringSingleton,
  }
}
