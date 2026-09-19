use core::ffi::c_void;

use crate::{
  functions::{
    are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type,
    seen_set_contains::seen_set_contains,
  },
  records::{
    are_equal_state::AreEqualState, type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
};
pub fn are_equal_are_equal_state_type_function_variadic_type_pack_type_function_variadic_type_pack(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionVariadicTypePack,
  rhs: &TypeFunctionVariadicTypePack,
) -> bool {
  if seen_set_contains(
    seen,
    lhs as *const TypeFunctionVariadicTypePack as *const c_void,
    rhs as *const TypeFunctionVariadicTypePack as *const c_void,
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
