use core::ffi::c_uint;

use crate::functions::lua_o_log_2::lua_o_log_2;

pub const fn ceillog2(x: c_uint) -> i32 {
  lua_o_log_2(x.wrapping_sub(1)) + 1
}
