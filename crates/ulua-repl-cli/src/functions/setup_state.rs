//! cpp `setupState`（`CLI/src/Repl.cpp`，经 `Repl.h` 暴露给测试使用）。

use core::ptr::null;

use ulua_cli_lib::functions::lua_collectgarbage::lua_collectgarbage;
use ulua_code_gen::functions::luau_codegen_create::luau_codegen_create;
use ulua_require::functions::luaopen_require::luaopen_require;
use ulua_vm::{
  functions::{
    lua_l_openlibs::lua_l_openlibs, lua_l_register::lua_l_register, lua_l_sandbox::lua_l_sandbox,
    lua_pushvalue::lua_pushvalue,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_pop::lua_pop},
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

use crate::functions::{
  create_cli_require_context::create_cli_require_context, lua_loadstring::lua_loadstring,
  repl_main::repl_codegen_enabled, require_config_init::require_config_init,
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
    lua_pushvalue(l, LUA_GLOBALSINDEX);
    lua_l_register(l, null(), &funcs);
    lua_pop(l, 1);
  }

  // Safety: l 有效；ctx 由 create_cli_require_context 返回（registry 持有），
  // require_config_init 签名匹配配置初始化回调。
  unsafe {
    let ctx = create_cli_require_context(l);
    luaopen_require(l, Some(require_config_init), ctx);
  }

  // Safety: 同上，l 有效。
  unsafe { lua_l_sandbox(l) };
}
