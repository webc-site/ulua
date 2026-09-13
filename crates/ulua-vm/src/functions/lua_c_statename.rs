use core::{
  ffi::{c_char, c_int},
  ptr::null,
};
pub(crate) const GCSPAUSE: i32 = 0;
pub(crate) const GCSPROPAGATE: i32 = 1;
pub(crate) const GCSPROPAGATEAGAIN: i32 = 2;
pub(crate) const GCSATOMIC: i32 = 3;
pub(crate) const GCSSWEEP: i32 = 4;

pub fn lua_c_statename(state: c_int) -> *const c_char {
  match state {
    GCSPAUSE => c"pause".as_ptr(),
    GCSPROPAGATE => c"mark".as_ptr(),
    GCSPROPAGATEAGAIN => c"remark".as_ptr(),
    GCSATOMIC => c"atomic".as_ptr(),
    GCSSWEEP => c"sweep".as_ptr(),
    _ => null(),
  }
}

pub use lua_c_statename as luaC_statename;
