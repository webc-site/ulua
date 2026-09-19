use core::ffi::c_void;
use std::collections::BTreeSet;
pub type SeenSet = BTreeSet<(*const c_void, *const c_void)>;
