use ulua_common::records::variant::Variant3;

use crate::records::{
  type_function_generic_type_pack::TypeFunctionGenericTypePack,
  type_function_type_pack::TypeFunctionTypePack,
  type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
};

pub type TypeFunctionTypePackVariant =
  Variant3<TypeFunctionTypePack, TypeFunctionVariadicTypePack, TypeFunctionGenericTypePack>;

/// `get_if<T>(&tv->type)` over the runtime pack variant (TypeFunctionRuntime.h:141).
pub trait TypeFunctionTypePackVariantMember: Sized {
  fn get_if(v: &TypeFunctionTypePackVariant) -> Option<&Self>;
  fn get_if_mut(v: &mut TypeFunctionTypePackVariant) -> Option<&mut Self>;
}

impl TypeFunctionTypePackVariantMember for TypeFunctionTypePack {
  fn get_if(v: &TypeFunctionTypePackVariant) -> Option<&Self> {
    v.get_if_0()
  }
  fn get_if_mut(v: &mut TypeFunctionTypePackVariant) -> Option<&mut Self> {
    v.get_if_0_mut()
  }
}

impl TypeFunctionTypePackVariantMember for TypeFunctionVariadicTypePack {
  fn get_if(v: &TypeFunctionTypePackVariant) -> Option<&Self> {
    v.get_if_1()
  }
  fn get_if_mut(v: &mut TypeFunctionTypePackVariant) -> Option<&mut Self> {
    v.get_if_1_mut()
  }
}

impl TypeFunctionTypePackVariantMember for TypeFunctionGenericTypePack {
  fn get_if(v: &TypeFunctionTypePackVariant) -> Option<&Self> {
    v.get_if_2()
  }
  fn get_if_mut(v: &mut TypeFunctionTypePackVariant) -> Option<&mut Self> {
    v.get_if_2_mut()
  }
}
