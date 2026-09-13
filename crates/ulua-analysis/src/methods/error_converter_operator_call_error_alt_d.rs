use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{error_converter::ErrorConverter, not_a_table::NotATable},
};
impl ErrorConverter {
  pub fn operator_call_34(&self, e: &NotATable) -> String {
    let ty = to_string_type_id(e.ty);
    String::from("Expected type table, got '") + &ty + "' instead"
  }
}
