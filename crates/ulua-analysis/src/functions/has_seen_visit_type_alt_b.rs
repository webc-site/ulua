use core::ffi::c_void;

use ulua_common::records::dense_hash_set::DenseHashSet;
pub fn has_seen_dense_hash_set_void_void(
  seen: &mut DenseHashSet<*mut c_void>,
  tv: *const c_void,
) -> bool {
  let ttv = tv as *mut c_void;

  if seen.contains(&ttv) {
    return true;
  }

  seen.insert(ttv);
  false
}
