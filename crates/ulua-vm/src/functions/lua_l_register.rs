//! Source: `VM/src/laux.cpp:304-327` (hand-ported)

use core::ffi::c_char;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    cstr_cow,
    libsize::libsize,
    lua_getfield::lua_getfield,
    lua_l_findtable::lua_l_findtable,
    lua_pushvalue::lua_pushvalue,
    lua_remove::lua_remove,
    lua_setfield::{lua_setfield, lua_setfield_bytes},
    lua_type::lua_type,
  },
  macros::{
    getstr::getstr, lua_globalsindex::LUA_GLOBALSINDEX, lua_l_error::luaL_error, lua_pop::lua_pop,
    lua_pushcfunction::LUA_PUSHCFUNCTION, lua_registryindex::LUA_REGISTRYINDEX,
    lua_s_new::lua_s_new,
  },
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 `LuaState`；`libname` 允许为 NULL（此时跳过建模块表），非空时须为 NUL 结尾 C 串；
/// `lr` 为 `LuaLReg` 纯切片，每项 `name` 为静态字节切片（不含尾部 `\0`）、`func` 为合法
/// C 函数指针；建表/注册可分配、`luaL_error` 可抛错，须受保护帧。cpp `laux.cpp:304`。
pub unsafe fn lua_l_register(l: *mut LuaState, libname: *const c_char, lr: &[LuaLReg]) {
  unsafe {
    if !libname.is_null() {
      let size = libsize(lr);
      lua_l_findtable(l, LUA_REGISTRYINDEX, c"_LOADED".as_ptr(), 1);
      lua_getfield(l, -1, libname);
      if lua_type(l, -1) != LuaType::Table as i32 {
        lua_pop(l, 1);
        if !lua_l_findtable(l, LUA_GLOBALSINDEX, libname, size).is_null() {
          let name = cstr_cow(libname);
          luaL_error!(l, "name conflict for module '{}'", name);
        }
        lua_pushvalue(l, -1);
        lua_setfield(l, -3, libname);
      }
      lua_remove(l, -2);
    }

    for reg in lr {
      let ts = lua_s_new(l, reg.name);
      LUA_PUSHCFUNCTION(l, reg.func, getstr(ts));
      lua_setfield_bytes(l, -2, reg.name);
    }
  }
}
