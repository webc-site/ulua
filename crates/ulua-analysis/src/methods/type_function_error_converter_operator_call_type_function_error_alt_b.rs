use alloc::string::String;

use ulua_common::functions::format::format;

use crate::{
  functions::to_string_to_string_alt_d::to_string_type_pack_id,
  records::{
    type_function_error_converter::TypeFunctionErrorConverter,
    unsupported_type_pack::UnsupportedTypePack,
  },
};

impl TypeFunctionErrorConverter {
  pub fn operator_call_5(&self, e: &UnsupportedTypePack) -> String {
    let pack_str = to_string_type_pack_id(e.pack);
    format(format_args!(
      "Type functions do not currently support types of the form '{}'",
      pack_str
    ))
  }
}
