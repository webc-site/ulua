use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_d::to_string_type_pack_id,
  records::{
    error_converter::ErrorConverter, uninhabited_type_pack_function::UninhabitedTypePackFunction,
  },
};

impl ErrorConverter {
  pub fn operator_call_61(&self, e: &UninhabitedTypePackFunction) -> String {
    format!(
      "Type pack function instance {} is uninhabited",
      to_string_type_pack_id(e.tp)
    )
  }
}
