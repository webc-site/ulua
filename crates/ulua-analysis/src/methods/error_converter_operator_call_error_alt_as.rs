use alloc::string::String;

use ulua_common::fflag;

use crate::{
  functions::{get_table_type::get_table_type, to_string_to_string_alt_c::to_string_type_id},
  records::{
    error_converter::ErrorConverter,
    property_access_violation::{PropertyAccessViolation, PropertyAccessViolation_Context},
  },
};

impl ErrorConverter {
  pub fn operator_call_50(&self, e: &PropertyAccessViolation) -> String {
    let mut chars = e.key().chars();
    let is_identifier_key = chars
      .next()
      .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
      && chars.all(|c| c.is_ascii_alphanumeric() || c == '_');

    let string_key = if is_identifier_key {
      e.key().to_string()
    } else {
      format!("\"{}\"", e.key())
    };

    if fflag::LuauTweakAccessViolationReporting.get() {
      let kind = if get_table_type(e.table()).is_some() {
        "table"
      } else {
        "type"
      };
      format!(
        "Property {string_key} of {kind} '{}' is {}-only",
        to_string_type_id(e.table()),
        match e.context() {
          PropertyAccessViolation_Context::CannotRead => "write",
          PropertyAccessViolation_Context::CannotWrite => "read",
        }
      )
    } else {
      let kind = match e.context() {
        PropertyAccessViolation_Context::CannotRead => "write",
        PropertyAccessViolation_Context::CannotWrite => "read",
      };
      format!(
        "Property {string_key} of table '{}' is {kind}-only",
        to_string_type_id(e.table())
      )
    }
  }
}
