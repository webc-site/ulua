//! cpp `setupState`（`CLI/src/Repl.cpp`，经 `Repl.h` 暴露给测试使用）。

use core::ptr::null;

use ulua_cli_lib::functions::lua_collectgarbage::lua_collectgarbage;
use ulua_code_gen::functions::luau_codegen_create::luau_codegen_create;
use ulua_require::functions::luaopen_require::luaopen_require;
use ulua_vm::{
  functions::{
    lua_l_openlibs::lua_l_openlibs, lua_l_register::lua_l_register, lua_l_sandbox::lua_l_sandbox,
  },
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

use crate::functions::{
  create_cli_require_context::create_cli_require_context, lua_loadstring::lua_loadstring,
  repl_main::repl_codegen_enabled,
};

/// # Safety
///
/// `l` 必须是刚创建、有效的 `LuaState`。
pub unsafe fn setup_state(l: *mut LuaState) {
  // Safety: l 为调用方（run_repl/repl_main 路径）刚创建的有效状态（fn /// # Safety）。
  if repl_codegen_enabled() {
    unsafe { luau_codegen_create(l) };
  }

  // Safety: 同上，lua_l_openlibs 仅初始化标准库表。
  unsafe { lua_l_openlibs(l) };

  // Note: a CALLGRIND build also registers {"callgrind", lua_callgrind}; the
  // upstream default (non-CALLGRIND) build registers only these two.
  // 注册表是纯数据构造，安全代码内完成
  let funcs: [LuaLReg; 2] = [
    LuaLReg::new(b"loadstring", lua_loadstring),
    LuaLReg::new(b"collectgarbage", lua_collectgarbage),
  ];

  // Safety: funcs 是本地数组，lua_l_register 在调用窗口内读取其 name/func 字段
  // （切片与 Some(fn) 指针均静态存活）；name 传 null 与 C++ 默认实参语义一致
  // （注册到当前全局表）；push/pop 配平收回 LUA_GLOBALSINDEX。
  unsafe {
    (*l).push_value(LUA_GLOBALSINDEX);
    lua_l_register(l, null(), &funcs);
    (*l).pop(1);
  }

  // Safety: l 有效；宿主随 require 闭包的 userdata 由 GC 持有（luaopen_require
  // 内部 lua_newuserdatadtor 装箱），与状态同生命周期。
  unsafe { luaopen_require(l, create_cli_require_context()) };

  // Safety: 同上，l 有效。
  unsafe { lua_l_sandbox(l) };
}
