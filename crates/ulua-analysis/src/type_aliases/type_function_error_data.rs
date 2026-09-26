use ulua_common::records::variant::Variant5;

use crate::{
  macros::variant_member,
  records::{
    failed_to_compile::FailedToCompile, runtime_error::RuntimeError,
    type_function_missing::TypeFunctionMissing, unsupported_type::UnsupportedType,
    unsupported_type_pack::UnsupportedTypePack,
  },
};

pub type TypeFunctionErrorData = Variant5<
  UnsupportedType,
  UnsupportedTypePack,
  RuntimeError,
  FailedToCompile,
  TypeFunctionMissing,
>;

variant_member! {
  /// `get_if<T>(&e.data)` over the error-data variant (TypeFunctionError.h:54).
  position TypeFunctionErrorDataMember: TypeFunctionErrorData {
    0 => UnsupportedType,
    1 => UnsupportedTypePack,
    2 => RuntimeError,
    3 => FailedToCompile,
    4 => TypeFunctionMissing,
  }
}
