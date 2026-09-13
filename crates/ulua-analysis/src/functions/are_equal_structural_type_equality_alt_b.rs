use core::{ffi::c_void, mem::transmute};
use std::collections::BTreeSet;

use crate::{
  functions::{
    are_equal_structural_type_equality::are_equal_seen_set_type_pack_var_type_pack_var,
    are_seen::are_seen,
  },
  records::function_type::FunctionType,
  type_aliases::seen_set_structural_type_equality::SeenSet,
};
pub fn are_equal_seen_set_function_type_function_type(
  seen: &mut SeenSet,
  lhs: &FunctionType,
  rhs: &FunctionType,
) -> bool {
  if are_seen(
    unsafe { transmute::<&mut SeenSet, &mut BTreeSet<(*mut c_void, *mut c_void)>>(seen) },
    lhs as *const FunctionType as *const c_void,
    rhs as *const FunctionType as *const c_void,
  ) {
    return true;
  }

  let lhs_arg_types = unsafe { &*lhs.arg_types };
  let rhs_arg_types = unsafe { &*rhs.arg_types };
  if !are_equal_seen_set_type_pack_var_type_pack_var(seen, lhs_arg_types, rhs_arg_types) {
    return false;
  }

  let lhs_ret_types = unsafe { &*lhs.ret_types };
  let rhs_ret_types = unsafe { &*rhs.ret_types };
  are_equal_seen_set_type_pack_var_type_pack_var(seen, lhs_ret_types, rhs_ret_types)
}
