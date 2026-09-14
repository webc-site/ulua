use core::ffi::c_void;

use crate::{
  functions::{
    are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type,
    seen_set_contains::seen_set_contains,
  },
  records::{are_equal_state::AreEqualState, type_function_table_type::TypeFunctionTableType},
};
pub fn are_equal_are_equal_state_type_function_table_type_type_function_table_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionTableType,
  rhs: &TypeFunctionTableType,
) -> bool {
  if seen_set_contains(
    seen,
    lhs as *const TypeFunctionTableType as *const c_void,
    rhs as *const TypeFunctionTableType as *const c_void,
  ) {
    return true;
  }

  if lhs.props.len() != rhs.props.len() {
    return false;
  }

  if (lhs.indexer.is_some()) != (rhs.indexer.is_some()) {
    return false;
  }

  if let (Some(l_indexer), Some(r_indexer)) = (&lhs.indexer, &rhs.indexer) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      unsafe { &*l_indexer.key_type },
      unsafe { &*r_indexer.key_type },
    ) {
      return false;
    }

    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      unsafe { &*l_indexer.value_type },
      unsafe { &*r_indexer.value_type },
    ) {
      return false;
    }
  }

  let mut l_iter = lhs.props.iter();
  let mut r_iter = rhs.props.iter();

  while let (Some((l_key, l_prop)), Some((r_key, r_prop))) = (l_iter.next(), r_iter.next()) {
    let _ = l_key;
    let _ = r_key;

    if (l_prop.read_ty.is_some() && r_prop.read_ty.is_none())
      || (l_prop.read_ty.is_none() && r_prop.read_ty.is_some())
    {
      return false;
    }

    if let (Some(l_read_ty), Some(r_read_ty)) = (&l_prop.read_ty, &r_prop.read_ty)
      && !are_equal_are_equal_state_type_function_type_type_function_type(
        seen,
        unsafe { &**l_read_ty },
        unsafe { &**r_read_ty },
      )
    {
      return false;
    }

    if (l_prop.write_ty.is_some() && r_prop.write_ty.is_none())
      || (l_prop.write_ty.is_none() && r_prop.write_ty.is_some())
    {
      return false;
    }

    if let (Some(l_write_ty), Some(r_write_ty)) = (&l_prop.write_ty, &r_prop.write_ty)
      && !are_equal_are_equal_state_type_function_type_type_function_type(
        seen,
        unsafe { &**l_write_ty },
        unsafe { &**r_write_ty },
      )
    {
      return false;
    }
  }

  true
}
