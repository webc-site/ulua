//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：对应 C++
//! `CLI/src/Repl.cpp:118` 的 `lua_collectgarbage`。两侧实现逻辑逐行相同。

use core::{
  ffi::{CStr, c_int},
  ptr::null_mut,
};

use ulua_vm::{
  enums::lua_gc_op::LuaGcOp,
  functions::{lua_gc::lua_gc, lua_l_optlstring::lua_l_optlstring, lua_pushnumber::lua_pushnumber},
  macros::lua_l_error::luaL_error,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// `l` 必须是有效、活跃的 `lua_State` 指针。
pub unsafe extern "C-unwind" fn lua_collectgarbage(l: *mut lua_State) -> c_int {
  unsafe {
    let option = lua_l_optlstring(l, 1, c"collect".as_ptr(), null_mut());
    let option = CStr::from_ptr(option);

    if option.to_bytes() == b"collect" {
      lua_gc(l, LuaGcOp::Collect as c_int, 0);
      return 0;
    }

    if option.to_bytes() == b"count" {
      let c = lua_gc(l, LuaGcOp::Count as c_int, 0);
      lua_pushnumber(l, c as f64);
      return 1;
    }

    // luaL_error! 恒发散（longjmp），无需回退值
    luaL_error!(l, "collectgarbage must be called with 'count' or 'collect'")
  }
}
