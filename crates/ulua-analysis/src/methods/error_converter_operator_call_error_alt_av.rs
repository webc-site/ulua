use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_d::to_string_type_pack_id,
  records::{
    error_converter::ErrorConverter,
    unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping,
  },
};

impl ErrorConverter {
  pub fn operator_call_59(&self, e: &UnexpectedTypePackInSubtyping) -> String {
    let tp_str = to_string_type_pack_id(e.tp);
    String::from("Encountered an unexpected type pack in subtyping: ") + &tp_str
  }
}
