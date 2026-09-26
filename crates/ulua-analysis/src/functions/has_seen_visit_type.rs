use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{records::visit_key::VisitKey, type_aliases::collections::HashSet};

pub fn has_seen(seen: &mut HashSet<VisitKey>, tv: *const ()) -> bool {
  !seen.insert(VisitKey::from_ptr(tv))
}

pub fn has_seen_dense_hash_set_void_void(seen: &mut DenseHashSet<VisitKey>, tv: *const ()) -> bool {
  let key = VisitKey::from_ptr(tv);

  if seen.contains(&key) {
    return true;
  }

  seen.insert(key);
  false
}
