use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{error_converter::ErrorConverter, optional_value_access::OptionalValueAccess},
};

impl ErrorConverter {
  pub fn operator_call_37(&self, e: &OptionalValueAccess) -> String {
    let ty = to_string_type_id(e.optional);
    String::from("Value of type '") + &ty + "' could be nil"
  }
}
