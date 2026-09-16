use core::ffi::c_void;

use ulua_common::records::dense_hash_set::DenseHashSet;
pub fn unsee_dense_hash_set_void_void(_seen: &mut DenseHashSet<*mut c_void>, _tv: *const c_void) {
  // When DenseHashSet is used for 'visitTypeOnce', where don't forget visited elements
}
