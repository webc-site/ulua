//! Faithful port of `lua_collectgarbage` from `CLI/src/Repl.cpp:118`.
//! Mirrors `luau-repl-cli/src/functions/lua_collectgarbage.rs`, adapted to the
//! cli-test module layout (`lua_State` lives under `records::lua_state`).

use core::{
  ffi::{CStr, c_int},
  ptr::null_mut,
};

use ulua_vm::{
  enums::lua_gc_op::LuaGcOp,
  functions::{lua_gc::lua_gc, lua_l_optlstring::lua_l_optlstring, lua_pushnumber::lua_pushnumber},
  macros::lua_l_error::luaL_error,
  records::lua_state::lua_State,
};

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

    luaL_error!(l, "collectgarbage must be called with 'count' or 'collect'");
    0
  }
}
