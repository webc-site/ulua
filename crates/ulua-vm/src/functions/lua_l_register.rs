//! Node: `cxx:Function:Luau.VM:VM/src/laux.cpp:304:luaL_register`
//! Source: `VM/src/laux.cpp:304-327` (hand-ported)

use core::ffi::{CStr, c_char};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    libsize::libsize, lua_getfield::lua_getfield, lua_l_error_l::lua_l_error_l,
    lua_l_findtable::luaL_findtable, lua_pushvalue::lua_pushvalue, lua_remove::lua_remove,
    lua_setfield::lua_setfield, lua_type::lua_type,
  },
  macros::{
    lua_globalsindex::LUA_GLOBALSINDEX, lua_pop::lua_pop, lua_pushcfunction::LUA_PUSHCFUNCTION,
    lua_registryindex::LUA_REGISTRYINDEX,
  },
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_register(l: *mut lua_State, libname: *const c_char, mut lr: *const LuaLReg) {
  unsafe {
    if !libname.is_null() {
      let size = libsize(lr);
      luaL_findtable(l, LUA_REGISTRYINDEX, c"_LOADED".as_ptr(), 1);
      lua_getfield(l, -1, libname);
      if lua_type(l, -1) != LuaType::Table as i32 {
        lua_pop(l, 1);
        if !luaL_findtable(l, LUA_GLOBALSINDEX, libname, size).is_null() {
          let name = CStr::from_ptr(libname).to_string_lossy();
          lua_l_error_l(
            l,
            c"name conflict for module '%s'".as_ptr(),
            format_args!("name conflict for module '{}'", name),
          );
        }
        lua_pushvalue(l, -1);
        lua_setfield(l, -3, libname);
      }
      lua_remove(l, -2);
    }

    while !(*lr).name.is_null() {
      LUA_PUSHCFUNCTION(l, (*lr).func, (*lr).name);
      lua_setfield(l, -2, (*lr).name);
      lr = lr.add(1);
    }
  }
}
