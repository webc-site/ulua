use crate::records::cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls;

impl CannotCheckDynamicStringFormatCalls {
  #[inline]
  pub fn operator_eq(&self, _other: &CannotCheckDynamicStringFormatCalls) -> bool {
    true
  }
}
