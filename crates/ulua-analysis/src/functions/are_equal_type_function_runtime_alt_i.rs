use core::ffi::c_void;

use crate::{
  functions::{
    are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type,
    seen_set_contains::seen_set_contains,
  },
  records::{
    are_equal_state::AreEqualState, type_function_negation_type::TypeFunctionNegationType,
  },
};
pub fn are_equal_are_equal_state_type_function_negation_type_type_function_negation_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionNegationType,
  rhs: &TypeFunctionNegationType,
) -> bool {
  if seen_set_contains(
    seen,
    lhs as *const TypeFunctionNegationType as *const c_void,
    rhs as *const TypeFunctionNegationType as *const c_void,
  ) {
    return true;
  }

  unsafe {
    are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      &*lhs.type_id,
      &*rhs.type_id,
    )
  }
}
