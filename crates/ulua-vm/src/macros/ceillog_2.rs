use core::ffi::{c_int, c_uint};

use crate::functions::lua_o_log_2::luaO_log2;

pub fn ceillog2(x: c_uint) -> c_int {
  luaO_log2(x.wrapping_sub(1)) + 1
}
