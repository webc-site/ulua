use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_d::to_string_type_pack_id,
  records::{error_converter::ErrorConverter, type_pack_mismatch::TypePackMismatch},
};

impl ErrorConverter {
  pub fn operator_call_54(&self, e: &TypePackMismatch) -> String {
    let wanted_str = to_string_type_pack_id(e.wanted_tp);
    let given_str = to_string_type_pack_id(e.given_tp);
    let mut ss =
      String::from("Expected this to be '") + &wanted_str + "', but got '" + &given_str + "'";

    if !e.reason.is_empty() {
      ss += "; ";
      ss += &e.reason;
    }

    ss
  }
}
