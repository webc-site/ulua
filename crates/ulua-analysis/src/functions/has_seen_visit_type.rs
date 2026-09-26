use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::type_aliases::collections::HashSet;

pub fn has_seen(seen: &mut HashSet<*mut ()>, tv: *const ()) -> bool {
  let ttv = tv as *mut ();
  !seen.insert(ttv)
}

pub fn has_seen_dense_hash_set_void_void(seen: &mut DenseHashSet<*mut ()>, tv: *const ()) -> bool {
  let ttv = tv as *mut ();

  if seen.contains(&ttv) {
    return true;
  }

  seen.insert(ttv);
  false
}
