use ulua_analysis::records::{
  check_result::CheckResult, checked_function_call_error::CheckedFunctionCallError,
};
use ulua_ast::records::position::Position;

use crate::functions::type_error_data_ref::type_error_data_ref;

pub fn require_non_strict_checked_error_at(result: &CheckResult, position: Position, name: &str) {
  for error in &result.errors {
    if error.location.begin == position {
      let Some(error) = type_error_data_ref::<CheckedFunctionCallError>(error) else {
        panic!("expected CheckedFunctionCallError at {:?}", position);
      };

      assert_eq!(name, error.checked_function_name());
      return;
    }
  }

  panic!("expected error at {:?}", position);
}
