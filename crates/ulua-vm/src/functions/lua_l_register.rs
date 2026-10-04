//! Source: `VM/src/laux.cpp:304-327` (hand-ported)

use alloc::string::String;

use crate::{
  functions::{libsize::libsize, lua_l_findtable::lua_l_findtable_bytes},
  macros::{
    getstr::getstr, lua_globalsindex::LUA_GLOBALSINDEX, lua_l_error::luaL_error,
    lua_registryindex::LUA_REGISTRYINDEX, lua_s_new::lua_s_new,
  },
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// # Safety
/// `l` 的存活与独占已由 `&mut LuaState` 引用形承载（本票收形）；`libname` 允许为
/// `None`（此时跳过建模块表）；`lr` 为 `LuaLReg` 纯切片，每项 `name` 为静态字节切片
/// （不含尾部 `\0`）、`func` 为合法 C 函数指针；建表/注册可分配、`luaL_error` 可抛错，
/// 须受保护帧。`unsafe fn` 屏障保留：体内仍把 `lr` 的裸 C 函数指针与 `lua_s_new`
/// 的裸形转手交出，真实裸操作未清零。
pub unsafe fn lua_l_register_bytes(l: &mut LuaState, libname: Option<&[u8]>, lr: &[LuaLReg]) {
  unsafe {
    if let Some(libname) = libname {
      let size = libsize(lr);
      lua_l_findtable_bytes(&mut *l, LUA_REGISTRYINDEX, b"_LOADED", 1);
      l.get_field_bytes(-1, libname);
      if !l.is_table(-1) {
        l.pop(1);
        if !lua_l_findtable_bytes(&mut *l, LUA_GLOBALSINDEX, libname, size).is_null() {
          let name = String::from_utf8_lossy(libname);
          luaL_error!(l, "name conflict for module '{}'", name);
        }
        l.push_value(-1);
        l.set_field_bytes(-3, libname);
      }
      l.remove(-2);
    }

    for reg in lr {
      let ts = lua_s_new(&mut *l, reg.name);
      l.push_c_function(reg.func, getstr(ts));
      l.set_field_bytes(-2, reg.name);
    }
  }
}
