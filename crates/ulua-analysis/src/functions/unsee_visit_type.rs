use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{records::visit_key::VisitKey, type_aliases::collections::HashSet};

pub fn unsee(seen: &mut HashSet<VisitKey>, tv: *const ()) {
  seen.remove(&VisitKey::from_ptr(tv));
}

pub fn unsee_dense_hash_set_void_void(_seen: &mut DenseHashSet<VisitKey>, _tv: *const ()) {
  // When DenseHashSet is used for 'visitTypeOnce', where don't forget visited elements
}
