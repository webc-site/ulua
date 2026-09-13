use core::ffi::c_void;

use ulua_common::records::dense_hash_set::DenseHashSet;
pub type SeenSet = DenseHashSet<*const c_void>;
