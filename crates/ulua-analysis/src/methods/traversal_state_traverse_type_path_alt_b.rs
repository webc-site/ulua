//! Source: `Analysis/src/TypePath.cpp:393-467` (hand-ported)
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    begin_type_pack::begin, end_type_pack::end, follow_type::follow_type_id,
    get_type_alt_j::get_type_id, get_type_or_pack::get_type_or_pack_mut as get_type_or_pack,
    get_type_or_pack_alt_s::get_type_or_pack_mut_2,
  },
  records::{
    index::Index, intersection_type::IntersectionType, traversal_state::TraversalState,
    type_iterator::TypeIterator, type_pack::TypePack, union_type::UnionType,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId, type_pack_id::TypePackId},
};

impl TraversalState {
  pub fn traverse_type_path_index(&mut self, index: &Index) -> bool {
    if self.check_invariants() {
      return false;
    }

    let current_type = get_type_or_pack::<TypeId>(&self.current);
    if !current_type.is_null() {
      let current_type_id = unsafe { follow_type_id(*current_type) };
      let mut updated_current = false;

      if get_type_id::<ErrorType>(current_type_id).is_some() {
        self.encountered_error_suppression = true;
        return false;
      }

      if let Some(u) = get_type_id::<UnionType>(current_type_id) {
        // We want to track the index that updates the current type with
        // `idx` while still iterating through the entire union to check
        // for error types.
        let mut idx: usize = 0;
        let mut it = unsafe { TypeIterator::<UnionType>::type_iterator_type(u) };
        let end_it = TypeIterator::<UnionType>::type_iterator_default();
        while it.operator_ne(&end_it) {
          let opt_ty = it.operator_deref();
          it.operator_inc();

          if get_type_id::<ErrorType>(opt_ty).is_some() {
            self.encountered_error_suppression = true;
          }
          if idx == index.index {
            self.update_current_type_id(opt_ty);
            updated_current = true;
          }
          idx += 1;
        }
      } else if let Some(i) = get_type_id::<IntersectionType>(current_type_id) {
        let mut idx: usize = 0;
        let mut it = unsafe { TypeIterator::<IntersectionType>::type_iterator_type(i) };
        let end_it = TypeIterator::<IntersectionType>::type_iterator_default();
        while it.operator_ne(&end_it) {
          let part_ty = it.operator_deref();
          it.operator_inc();

          if get_type_id::<ErrorType>(part_ty).is_some() {
            self.encountered_error_suppression = true;
          }
          if idx == index.index {
            self.update_current_type_id(part_ty);
            updated_current = true;
          }
          idx += 1;
        }
      }

      updated_current
    } else {
      let current_pack = get_type_or_pack::<TypePackId>(&self.current);
      LUAU_ASSERT!(!current_pack.is_null());
      if !get_type_or_pack_mut_2::<TypePack>(&self.current).is_null() {
        let cp: TypePackId = unsafe { *current_pack };
        let mut it = begin(cp);
        let mut i: usize = 0;
        while i < index.index && it.operator_ne(&end(cp)) {
          it.operator_inc();
          i += 1;
        }

        if it.operator_ne(&end(cp)) {
          self.update_current_type_id(*it.operator_deref());
          return true;
        }
      }

      false
    }
  }
}
