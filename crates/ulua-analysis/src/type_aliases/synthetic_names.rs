use core::ffi::{c_char, c_void};

use ulua_common::records::dense_hash_map::DenseHashMap;
pub type SyntheticNames = DenseHashMap<*const c_void, *mut c_char>;
