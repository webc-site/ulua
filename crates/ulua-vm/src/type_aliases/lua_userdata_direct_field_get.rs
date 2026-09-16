use core::ffi::c_void;
pub type LuaUserdataDirectFieldGet =
  Option<unsafe extern "C-unwind" fn(ud: *mut c_void, result: *mut c_void)>;
