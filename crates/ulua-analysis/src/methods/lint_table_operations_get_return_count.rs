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

    if let Some(ftv) = get_type_id::<FunctionType>(ty) {
      return unsafe { size(ftv.ret_types, null_mut()) };
    }

    if let Some(itv) = get_type_id::<IntersectionType>(ty) {
      // We don't process the type recursively to avoid having to deal with
      // self-recursive intersection types
      let mut result = 0;

      for &part in itv.parts.iter() {
        let followed_part = follow_type_id(part);
        if let Some(ftv) = get_type_id::<FunctionType>(followed_part) {
          let count = unsafe { size(ftv.ret_types, null_mut()) };
          result = max(result, count);
        }
      }

      return result;
    }

    0
  }
}
