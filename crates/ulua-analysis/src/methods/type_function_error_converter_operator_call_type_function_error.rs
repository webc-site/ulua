use alloc::string::String;

use ulua_common::functions::format::format;

use crate::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{
    type_function_error_converter::TypeFunctionErrorConverter, unsupported_type::UnsupportedType,
  },
};

impl TypeFunctionErrorConverter {
  pub fn operator_call_4(&self, e: &UnsupportedType) -> String {
    let ty = to_string_type_id(e.r#type);
    format(format_args!(
      "Type functions do not currently support types of the form '{}'",
      ty
    ))
  }
}
