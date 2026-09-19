use alloc::string::String;

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    to_string_to_string_alt_c::to_string_type_id,
  },
  records::{
    error_converter::ErrorConverter, extern_type::ExternType, table_type::TableType,
    unknown_property::UnknownProperty,
  },
};

impl ErrorConverter {
  pub fn operator_call_43(&self, e: &UnknownProperty) -> String {
    let t = follow_type_id(e.table);
    if get_type_id::<TableType>(t).is_none() {
      if get_type_id::<ExternType>(t).is_none() {
        format!(
          "Type '{}' does not have key '{}'",
          to_string_type_id(e.table),
          e.key
        )
      } else {
        format!(
          "Key '{}' not found in external type '{}'",
          e.key,
          to_string_type_id(t)
        )
      }
    } else {
      format!(
        "Key '{}' not found in table '{}'",
        e.key,
        to_string_type_id(t)
      )
    }
  }
}
