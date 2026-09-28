use core::ffi::c_void;
pub type LuaCounterValue =
  Option<unsafe extern "C-unwind" fn(context: *mut c_void, kind: i32, line: i32, hits: u64)>;
