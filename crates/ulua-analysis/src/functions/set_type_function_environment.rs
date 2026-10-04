//! Faithful port of `void setTypeFunctionEnvironment(lua_State* l)`
//! (Analysis/src/TypeFunctionRuntime.cpp:2098-2138).
//!
//! Adds libraries / globals for the type-function environment: opens a curated
//! subset of the standard libraries, replaces a handful of unavailable base
//! globals with a stub that errors, and installs the custom `print`.
// `LuaCfunction` thunk for `unsupportedFunction`. Bridges the analysis-level
// function (over the opaque `LuaState` struct) to the VM's `LuaCfunction`
// shape (`unsafe fn(*mut vm::lua_State) -> i32`).

use core::ptr::null;

use ulua_vm::{
  functions::{
    luaopen_base::luaopen_base, luaopen_bit_32::luaopen_bit32, luaopen_buffer::luaopen_buffer,
    luaopen_math::luaopen_math, luaopen_string::luaopen_string, luaopen_table::luaopen_table,
    luaopen_utf_8::luaopen_utf_8,
  },
  records::{lua_state, lua_state::LuaState},
};

use crate::functions::{
  lua_names::{
    GLOBAL_GCINFO, GLOBAL_GETFENV, GLOBAL_NEWPROXY, GLOBAL_PCALL, GLOBAL_PRINT, GLOBAL_SETFENV,
    GLOBAL_XPCALL,
  },
  print::print,
  unsupported_function::unsupported_function,
};
unsafe extern "C-unwind" fn unsupported_function_thunk(l: *mut lua_state::LuaState) -> i32 {
  unsafe { unsupported_function(&mut *l) }
}

/// `LuaCfunction` thunk for `print`.
unsafe extern "C-unwind" fn print_thunk(l: *mut lua_state::LuaState) -> i32 {
  unsafe { print(&mut *l) }
}

/// 对应 C++ `void setTypeFunctionEnvironment(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:2094`）。
///
/// 本函数是 safe fn：形参为 `&mut LuaState`，存活/独占由引用类型承载；体内 `unsafe`
/// 块只因 `luaopen_*` 是 VM 侧 C-ABI 门面（收 `*mut`，经 `l.as_mut_ptr()` 就地派生），
/// 调用方无需承担任何内存安全前提。
///
/// 调用序契约（正确性，非内存安全）：`l` 须是即将承载类型函数库、且尚未重复初始化的
/// 主线程状态；本函数在其上创建并注册 `TypeFunctionRuntime` userdatum 与全部原生函数
/// 闭包，重复调用会覆盖注册（非 UB）。
pub(crate) fn set_type_function_environment(l: &mut LuaState) {
  unsafe {
    // Register math library
    luaopen_math(l.as_mut_ptr());
    l.pop(1);

    // Register table library
    luaopen_table(l.as_mut_ptr());
    l.pop(1);

    // Register string library
    luaopen_string(l.as_mut_ptr());
    l.pop(1);

    // Register bit32 library
    luaopen_bit32(l.as_mut_ptr());
    l.pop(1);

    // Register utf8 library
    luaopen_utf_8(l);
    l.pop(1);

    // Register Buffer library
    luaopen_buffer(l.as_mut_ptr());
    l.pop(1);

    // Register base library
    luaopen_base(l.as_mut_ptr());
    l.pop(1);

    // Remove certain global functions from the base library
    // static const char* unavailableGlobals[] = {"gcinfo", "getfenv", "newproxy", "setfenv", "pcall", "xpcall"};
    let unavailable_globals: [&[u8]; 6] = [
      GLOBAL_GCINFO,
      GLOBAL_GETFENV,
      GLOBAL_NEWPROXY,
      GLOBAL_SETFENV,
      GLOBAL_PCALL,
      GLOBAL_XPCALL,
    ];
    for &name in &unavailable_globals {
      l.push_c_function(Some(unsupported_function_thunk), null());
      l.set_global_bytes(name);
    }

    l.push_c_function(Some(print_thunk), null());
    l.set_global_bytes(GLOBAL_PRINT);
  }
}
