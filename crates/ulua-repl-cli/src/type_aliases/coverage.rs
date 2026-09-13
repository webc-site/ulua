use core::ffi::{c_int, c_void};

pub type Coverage = Option<unsafe extern "C-unwind" fn(*mut c_void, c_int)>;
