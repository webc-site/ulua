//! Node: `cxx:Function:Luau.VM:VM/src/linit.cpp:42:luaL_openlibs`
//! Source: `VM/src/linit.cpp:42-60` (hand-ported)

use core::ptr::null;

use ulua_common::FFlag;

use crate::{
  functions::{
    lua_call::lua_call, lua_pushstring::lua_pushstring, luaopen_base::luaopen_base,
    luaopen_bit_32::luaopen_bit32, luaopen_buffer::luaopen_buffer, luaopen_class::luaopen_class,
    luaopen_coroutine::luaopen_coroutine, luaopen_debug::luaopen_debug,
    luaopen_integer::luaopen_integer, luaopen_math::luaopen_math, luaopen_os::luaopen_os,
    luaopen_string::luaopen_string, luaopen_table::luaopen_table, luaopen_utf_8::luaopen_utf_8,
    luaopen_vector::luaopen_vector,
  },
  macros::lua_pushcfunction::LUA_PUSHCFUNCTION,
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_openlibs(l: *mut lua_State) {
  unsafe {
    let lualibs = [
      LuaLReg {
        name: c"".as_ptr(),
        func: Some(luaopen_base),
      },
      LuaLReg {
        name: c"coroutine".as_ptr(),
        func: Some(luaopen_coroutine),
      },
      LuaLReg {
        name: c"table".as_ptr(),
        func: Some(luaopen_table),
      },
      LuaLReg {
        name: c"os".as_ptr(),
        func: Some(luaopen_os),
      },
      LuaLReg {
        name: c"string".as_ptr(),
        func: Some(luaopen_string),
      },
      LuaLReg {
        name: c"math".as_ptr(),
        func: Some(luaopen_math),
      },
      LuaLReg {
        name: c"debug".as_ptr(),
        func: Some(luaopen_debug),
      },
      LuaLReg {
        name: c"utf8".as_ptr(),
        func: Some(luaopen_utf_8),
      },
      LuaLReg {
        name: c"bit32".as_ptr(),
        func: Some(luaopen_bit32),
      },
      LuaLReg {
        name: c"buffer".as_ptr(),
        func: Some(luaopen_buffer),
      },
      LuaLReg {
        name: c"vector".as_ptr(),
        func: Some(luaopen_vector),
      },
      LuaLReg {
        name: c"int64".as_ptr(),
        func: Some(luaopen_integer),
      },
      LuaLReg {
        name: null(),
        func: None,
      },
    ];

    let lualibs_nointeger = [
      LuaLReg {
        name: c"".as_ptr(),
        func: Some(luaopen_base),
      },
      LuaLReg {
        name: c"coroutine".as_ptr(),
        func: Some(luaopen_coroutine),
      },
      LuaLReg {
        name: c"table".as_ptr(),
        func: Some(luaopen_table),
      },
      LuaLReg {
        name: c"os".as_ptr(),
        func: Some(luaopen_os),
      },
      LuaLReg {
        name: c"string".as_ptr(),
        func: Some(luaopen_string),
      },
      LuaLReg {
        name: c"math".as_ptr(),
        func: Some(luaopen_math),
      },
      LuaLReg {
        name: c"debug".as_ptr(),
        func: Some(luaopen_debug),
      },
      LuaLReg {
        name: c"utf8".as_ptr(),
        func: Some(luaopen_utf_8),
      },
      LuaLReg {
        name: c"bit32".as_ptr(),
        func: Some(luaopen_bit32),
      },
      LuaLReg {
        name: c"buffer".as_ptr(),
        func: Some(luaopen_buffer),
      },
      LuaLReg {
        name: c"vector".as_ptr(),
        func: Some(luaopen_vector),
      },
      LuaLReg {
        name: null(),
        func: None,
      },
    ];

    let libs = if FFlag::LuauIntegerLibrary.get() {
      lualibs.as_ptr()
    } else {
      lualibs_nointeger.as_ptr()
    };

    let mut lib = libs;
    while (*lib).func.is_some() {
      LUA_PUSHCFUNCTION(l, (*lib).func, null());
      lua_pushstring(l, (*lib).name);
      lua_call(l, 1, 0);
      lib = lib.add(1);
    }

    if FFlag::DebugLuauUserDefinedClassesRuntime.get() {
      LUA_PUSHCFUNCTION(l, Some(luaopen_class), null());
      lua_pushstring(l, c"class".as_ptr());
      lua_call(l, 1, 0);
    }
  }
}
