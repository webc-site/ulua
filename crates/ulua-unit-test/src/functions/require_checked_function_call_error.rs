use ulua_analysis::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{check_result::CheckResult, checked_function_call_error::CheckedFunctionCallError},
};

use crate::functions::type_error_data_ref::type_error_data_ref;

pub fn require_checked_function_call_error(
  result: &CheckResult,
  index: usize,
  name: &str,
  expected: &str,
  passed: &str,
) {
  let error = type_error_data_ref::<CheckedFunctionCallError>(&result.errors[index])
    .expect("expected CheckedFunctionCallError");

  assert_eq!(name, error.checked_function_name());
  assert_eq!(expected, to_string_type_id(error.expected()));
  assert_eq!(passed, to_string_type_id(error.passed()));
}
