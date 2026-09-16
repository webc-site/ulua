//! Faithful port of `void setTypeFunctionEnvironment(lua_State* l)`
//! (Analysis/src/TypeFunctionRuntime.cpp:2098-2138).
//!
//! Adds libraries / globals for the type-function environment: opens a curated
//! subset of the standard libraries, replaces a handful of unavailable base
//! globals with a stub that errors, and installs the custom `print`.
/// `LuaCfunction` thunk for `unsupportedFunction`. Bridges the analysis-level
/// function (over the `c_void` `lua_State` alias) to the VM's `LuaCfunction`
/// shape (`unsafe fn(*mut vm::lua_State) -> c_int`).
use core::ffi::c_char;
use core::ffi::c_int;

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
  functions::{print::print, unsupported_function::unsupported_function},
  type_aliases::lua_state::LuaState,
};
unsafe extern "C-unwind" fn unsupported_function_thunk(l: *mut lua_state::LuaState) -> c_int {
  unsafe { unsupported_function(l as *mut LuaState) }
}

/// `LuaCfunction` thunk for `print`.
unsafe extern "C-unwind" fn print_thunk(l: *mut lua_state::LuaState) -> c_int {
  unsafe { print(l as *mut LuaState) }
}

/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
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
    let unavailable_globals: [*const c_char; 6] = [
      c"gcinfo".as_ptr(),
      c"getfenv".as_ptr(),
      c"newproxy".as_ptr(),
      c"setfenv".as_ptr(),
      c"pcall".as_ptr(),
      c"xpcall".as_ptr(),
    ];
    for name in unavailable_globals.iter() {
      LUA_PUSHCFUNCTION(vm_l, Some(unsupported_function_thunk), *name);
      lua_setglobal(vm_l, *name);
    }

    LUA_PUSHCFUNCTION(vm_l, Some(print_thunk), c"print".as_ptr());
    lua_setglobal(vm_l, c"print".as_ptr());
  }
}
