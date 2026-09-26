use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::type_aliases::collections::HashSet;

pub fn unsee(seen: &mut HashSet<*mut ()>, tv: *const ()) {
  let ttv = tv as *mut ();
  seen.remove(&ttv);
}

pub fn unsee_dense_hash_set_void_void(_seen: &mut DenseHashSet<*mut ()>, _tv: *const ()) {
  // When DenseHashSet is used for 'visitTypeOnce', where don't forget visited elements
}
