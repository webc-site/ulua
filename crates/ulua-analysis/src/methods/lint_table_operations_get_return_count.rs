use core::ptr::null_mut;
use std::cmp::max;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, size_type_pack::size},
  records::{
    function_type::FunctionType, intersection_type::IntersectionType,
    lint_table_operations::LintTableOperations,
  },
  type_aliases::type_id::TypeId,
};
impl LintTableOperations {
  pub fn get_return_count(&mut self, ty: TypeId) -> usize {
    let ty = follow_type_id(ty);

    if !get_type_id::<FunctionType>(ty).is_none() {
      let ftv = get_type_id::<FunctionType>(ty).unwrap();
      return unsafe { size(ftv.ret_types, null_mut()) };
    }

    if !get_type_id::<IntersectionType>(ty).is_none() {
      let itv = get_type_id::<IntersectionType>(ty).unwrap();
      let mut result = 0;

      for &part in itv.parts.iter() {
        let followed_part = follow_type_id(part);
        if !get_type_id::<FunctionType>(followed_part).is_none() {
          let ftv = get_type_id::<FunctionType>(followed_part).unwrap();
          let count = unsafe { size(ftv.ret_types, null_mut()) };
          result = max(result, count);
        }
      }

      return result;
    }

    0
  }
}
