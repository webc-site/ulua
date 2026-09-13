//! Source: `Analysis/src/TypePath.cpp:595-627` (hand-ported)
use alloc::vec::Vec;

use crate::{
  functions::{
    flatten_type_pack_alt_b::flatten, get_type_or_pack::get_type_or_pack_mut as get_type_or_pack,
  },
  records::{pack_slice::PackSlice, traversal_state::TraversalState, txn_log::TxnLog},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl TraversalState {
  pub fn traverse_type_path_pack_slice(&mut self, slice: PackSlice) -> bool {
    if self.check_invariants() {
      return false;
    }

    let current_pack = { get_type_or_pack::<TypePackId>(&self.current) };
    if current_pack.is_null() {
      return false;
    }
    let cp: TypePackId = unsafe { *current_pack };

    let (flat_head, flat_tail) = flatten(cp, unsafe { &*TxnLog::empty() });

    if flat_head.len() <= slice.start_index {
      return false;
    }

    let mut head_slice: Vec<TypeId> = Vec::with_capacity(flat_head.len() - slice.start_index);

    // C++ walks `begin(flatHead)` advanced by `start_index` to `end(flatHead)`;
    // `flatHead` is a plain vector, so we slice it directly.
    for ty in flat_head.iter().skip(slice.start_index) {
      head_slice.push(*ty);
    }

    let pack_slice = unsafe {
      (*self.arena).add_type_pack_vector_type_id_optional_type_pack_id(head_slice, flat_tail)
    };

    self.update_current_type_pack_id(pack_slice);

    true
  }
}
