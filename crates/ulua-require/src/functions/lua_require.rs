use core::{
  ffi::{CStr, c_char},
  mem::zeroed,
};

use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  macros::lua_l_error::luaL_error,
  records::{lua_debug::LuaDebug, lua_state::lua_State},
};

use crate::functions::lua_requireinternal::lua_requireinternal;

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
pub unsafe extern "C-unwind" fn lua_require(l: *mut lua_State) -> i32 {
  unsafe {
    let mut ar: LuaDebug = zeroed();
    let mut level = 1;

    loop {
      if lua_getinfo(l, level, c"s".as_ptr(), &mut ar) == 0 {
        luaL_error!(l, "require is not supported in this context");
      }
      level += 1;

      if ar.what.is_null() || *ar.what != b'C' as c_char {
        break;
      }
    }

    // ar.source 为空时退化为空 chunkname（cpp 直接传指针，Rust 保底避免 UB）
    let requirer_chunkname = if ar.source.is_null() {
      c""
    } else {
      CStr::from_ptr(ar.source)
    };

    lua_requireinternal(l, requirer_chunkname)
  }
}
