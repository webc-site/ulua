//! Faithful port of `void setTypeFunctionEnvironment(lua_State* l)`
//! (Analysis/src/TypeFunctionRuntime.cpp:2098-2138).
//!
//! Adds libraries / globals for the type-function environment: opens a curated
//! subset of the standard libraries, replaces a handful of unavailable base
//! globals with a stub that errors, and installs the custom `print`.
// `LuaCfunction` thunk for `unsupportedFunction`. Bridges the analysis-level
// function (over the opaque `LuaState` struct) to the VM's `LuaCfunction`
// shape (`unsafe fn(*mut vm::lua_State) -> i32`).
use ulua_vm::{
  functions::{
    luaopen_base::luaopen_base, luaopen_bit_32::luaopen_bit32, luaopen_buffer::luaopen_buffer,
    luaopen_math::luaopen_math, luaopen_string::luaopen_string, luaopen_table::luaopen_table,
    luaopen_utf_8::luaopen_utf_8,
  },
  macros::{lua_pop::lua_pop, lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal},
  records::lua_state,
};

use crate::{
  functions::{
    lua_names::{
      GLOBAL_GCINFO, GLOBAL_GETFENV, GLOBAL_NEWPROXY, GLOBAL_PCALL, GLOBAL_PRINT, GLOBAL_SETFENV,
      GLOBAL_XPCALL,
    },
    print::print,
    unsupported_function::unsupported_function,
  },
  type_aliases::lua_state::LuaState,
};
unsafe extern "C-unwind" fn unsupported_function_thunk(l: *mut lua_state::LuaState) -> i32 {
  unsafe { unsupported_function(l as *mut LuaState) }
}

/// `LuaCfunction` thunk for `print`.
unsafe extern "C-unwind" fn print_thunk(l: *mut lua_state::LuaState) -> i32 {
  unsafe { print(l as *mut LuaState) }
}

/// # Safety
/// `l` 必须是即将承载类型函数库、且尚未重复初始化的主 `lua_State*`；本函数在其上创建并注册
/// `TypeFunctionRuntime` userdatum 与全部原生函数闭包，要求该状态在 VM 生命周期内不被并发访问。
/// 对应 C++ `void setTypeFunctionEnvironment(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:2094`）。
pub unsafe fn set_type_function_environment(l: *mut LuaState) {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;

    // Register math library
    luaopen_math(vm_l);
    lua_pop(vm_l, 1);

    // Register table library
    luaopen_table(vm_l);
    lua_pop(vm_l, 1);

    // Register string library
    luaopen_string(vm_l);
    lua_pop(vm_l, 1);

    // Register bit32 library
    luaopen_bit32(vm_l);
    lua_pop(vm_l, 1);

    // Register utf8 library
    luaopen_utf_8(vm_l);
    lua_pop(vm_l, 1);

    // Register Buffer library
    luaopen_buffer(vm_l);
    lua_pop(vm_l, 1);

    // Register base library
    luaopen_base(vm_l);
    lua_pop(vm_l, 1);

    // Remove certain global functions from the base library
    // static const char* unavailableGlobals[] = {"gcinfo", "getfenv", "newproxy", "setfenv", "pcall", "xpcall"};
    let unavailable_globals = [
      GLOBAL_GCINFO.as_ptr().cast(),
      GLOBAL_GETFENV.as_ptr().cast(),
      GLOBAL_NEWPROXY.as_ptr().cast(),
      GLOBAL_SETFENV.as_ptr().cast(),
      GLOBAL_PCALL.as_ptr().cast(),
      GLOBAL_XPCALL.as_ptr().cast(),
    ];
    for name in unavailable_globals.iter() {
      LUA_PUSHCFUNCTION(vm_l, Some(unsupported_function_thunk), *name);
      lua_setglobal(vm_l, *name);
    }

    LUA_PUSHCFUNCTION(vm_l, Some(print_thunk), GLOBAL_PRINT.as_ptr().cast());
    lua_setglobal(vm_l, GLOBAL_PRINT.as_ptr().cast());
  }
}
