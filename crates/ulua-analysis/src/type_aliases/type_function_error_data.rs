use ulua_common::records::variant::Variant5;

use crate::records::{
  failed_to_compile::FailedToCompile, runtime_error::RuntimeError,
  type_function_missing::TypeFunctionMissing, unsupported_type::UnsupportedType,
  unsupported_type_pack::UnsupportedTypePack,
};

pub type TypeFunctionErrorData = Variant5<
  UnsupportedType,
  UnsupportedTypePack,
  RuntimeError,
  FailedToCompile,
  TypeFunctionMissing,
>;

/// `get_if<T>(&e.data)` over the error-data variant (TypeFunctionError.h:54).
pub trait TypeFunctionErrorDataMember: Sized {
  fn get_if(v: &TypeFunctionErrorData) -> Option<&Self>;
  fn get_if_mut(v: &mut TypeFunctionErrorData) -> Option<&mut Self>;
}

impl TypeFunctionErrorDataMember for UnsupportedType {
  fn get_if(v: &TypeFunctionErrorData) -> Option<&Self> {
    v.get_if_0()
  }
  fn get_if_mut(v: &mut TypeFunctionErrorData) -> Option<&mut Self> {
    v.get_if_0_mut()
  }
}

impl TypeFunctionErrorDataMember for UnsupportedTypePack {
  fn get_if(v: &TypeFunctionErrorData) -> Option<&Self> {
    v.get_if_1()
  }
  fn get_if_mut(v: &mut TypeFunctionErrorData) -> Option<&mut Self> {
    v.get_if_1_mut()
  }
}

impl TypeFunctionErrorDataMember for RuntimeError {
  fn get_if(v: &TypeFunctionErrorData) -> Option<&Self> {
    v.get_if_2()
  }
  fn get_if_mut(v: &mut TypeFunctionErrorData) -> Option<&mut Self> {
    v.get_if_2_mut()
  }
}

impl TypeFunctionErrorDataMember for FailedToCompile {
  fn get_if(v: &TypeFunctionErrorData) -> Option<&Self> {
    v.get_if_3()
  }
  fn get_if_mut(v: &mut TypeFunctionErrorData) -> Option<&mut Self> {
    v.get_if_3_mut()
  }
}

impl TypeFunctionErrorDataMember for TypeFunctionMissing {
  fn get_if(v: &TypeFunctionErrorData) -> Option<&Self> {
    v.get_if_4()
  }
  fn get_if_mut(v: &mut TypeFunctionErrorData) -> Option<&mut Self> {
    v.get_if_4_mut()
  }
}
