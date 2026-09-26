use ulua_common::records::variant::Variant3;

use crate::{
  macros::variant_member,
  records::{
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
};

pub type TypeFunctionTypePackVariant =
  Variant3<TypeFunctionTypePack, TypeFunctionVariadicTypePack, TypeFunctionGenericTypePack>;

variant_member! {
  /// `get_if<T>(&tv->type)` over the runtime pack variant (TypeFunctionRuntime.h:141).
  position TypeFunctionTypePackVariantMember: TypeFunctionTypePackVariant {
    0 => TypeFunctionTypePack,
    1 => TypeFunctionVariadicTypePack,
    2 => TypeFunctionGenericTypePack,
  }
}
