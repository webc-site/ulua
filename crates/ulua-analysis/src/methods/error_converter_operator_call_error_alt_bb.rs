use alloc::string::String;

use crate::records::{
  cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls,
  error_converter::ErrorConverter,
};

impl ErrorConverter {
  pub fn operator_call_4(&self, _e: &CannotCheckDynamicStringFormatCalls) -> String {
    String::from(
      "We cannot statically check the type of `string.format` when called with a format string that is not statically known.\nIf you'd like to use an unchecked `string.format` call, you can cast the format string to `any` using `:: any`.",
    )
  }
}
