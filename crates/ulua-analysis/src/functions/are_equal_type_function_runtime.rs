use core::ffi::c_void;

use ulua_common::records::variant::Variant2;

use crate::{
  functions::seen_set_contains::seen_set_contains,
  records::{
    are_equal_state::AreEqualState, type_function_singleton_type::TypeFunctionSingletonType,
  },
};
pub fn are_equal_are_equal_state_type_function_singleton_type_type_function_singleton_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionSingletonType,
  rhs: &TypeFunctionSingletonType,
) -> bool {
  if seen_set_contains(
    seen,
    lhs as *const TypeFunctionSingletonType as *const c_void,
    rhs as *const TypeFunctionSingletonType as *const c_void,
  ) {
    return true;
  }

  match (&lhs.variant, &rhs.variant) {
    (Variant2::V0(lp), Variant2::V0(rp)) => {
      return lp.value == rp.value;
    }
    (Variant2::V1(lp), Variant2::V1(rp)) => {
      return lp.value == rp.value;
    }
    _ => {}
  }

  false
}
