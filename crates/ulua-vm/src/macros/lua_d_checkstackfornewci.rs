use core::ffi::c_int;

use crate::{
  functions::lua_d_reallocstack::lua_d_reallocstack,
  macros::{
    condhardstacktests::condhardstacktests, getgrownstacksize::getgrownstacksize,
    stacklimitreached::stacklimitreached,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `LuaState`.
#[inline]
pub unsafe fn lua_d_checkstackfornewci(l: *mut LuaState, n: c_int) {
  unsafe {
    if stacklimitreached(l, n) {
      lua_d_reallocstack(l, getgrownstacksize(l, n), 1);
    } else {
      condhardstacktests!(lua_d_reallocstack(l, (*l).stacksize - EXTRA_STACK, 1));
    }
  }
}

pub use lua_d_checkstackfornewci as luaD_checkstackfornewci;
