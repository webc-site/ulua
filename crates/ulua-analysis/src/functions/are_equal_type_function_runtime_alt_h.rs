use core::ffi::c_void;

use crate::{
  functions::{
    are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type,
    seen_set_contains::seen_set_contains,
  },
  records::{
    are_equal_state::AreEqualState, type_function_intersection_type::TypeFunctionIntersectionType,
  },
};
pub fn are_equal_are_equal_state_type_function_intersection_type_type_function_intersection_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionIntersectionType,
  rhs: &TypeFunctionIntersectionType,
) -> bool {
  if seen_set_contains(
    seen,
    lhs as *const TypeFunctionIntersectionType as *const c_void,
    rhs as *const TypeFunctionIntersectionType as *const c_void,
  ) {
    return true;
  }

  if lhs.components.len() != rhs.components.len() {
    return false;
  }

  // components 按下标一一对应，zip 替代索引遍历
  for (&l, &r) in lhs.components.iter().zip(&rhs.components) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      unsafe { &*l },
      unsafe { &*r },
    ) {
      return false;
    }
  }

  true
}
