use core::ffi::c_void;

use crate::{
  functions::{
    are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type,
    seen_set_contains::seen_set_contains,
  },
  records::{are_equal_state::AreEqualState, type_function_union_type::TypeFunctionUnionType},
};
pub fn are_equal_are_equal_state_type_function_union_type_type_function_union_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionUnionType,
  rhs: &TypeFunctionUnionType,
) -> bool {
  if seen_set_contains(
    seen,
    lhs as *const TypeFunctionUnionType as *const c_void,
    rhs as *const TypeFunctionUnionType as *const c_void,
  ) {
    return true;
  }

  if lhs.components.len() != rhs.components.len() {
    return false;
  }

  let mut l_iter = lhs.components.iter();
  let mut r_iter = rhs.components.iter();

  while let (Some(l), Some(r)) = (l_iter.next(), r_iter.next()) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      unsafe { &**l },
      unsafe { &**r },
    ) {
      return false;
    }
  }

  true
}
