//! ReplFixture / ReplWithPathFixture 公共状态搭建（对齐 cpp 两个 fixture 构造
//! 函数相同的 luaL_newstate → setupState → luaL_sandboxthread → runCode
//! (prettyPrintSource) 序列）。

use ulua_repl_cli::functions::{run_code::run_code, setup_state::setup_state};
use ulua_vm::{
  functions::{lua_l_newstate::lua_l_newstate, lua_l_sandboxthread::lua_l_sandboxthread},
  type_aliases::lua_state::lua_State,
};

/// 创建沙箱化状态并执行 `pretty_print` 源码，返回主线程状态。
///
/// # Safety
/// 返回的 `lua_State` 由调用方（fixture）负责 `lua_close`；`pretty_print`
/// 必须是合法 Lua 源码。
pub(crate) unsafe fn new_fixture_state(pretty_print: &str) -> *mut lua_State {
  unsafe {
    let l_state = lua_l_newstate();

    setup_state(l_state);

    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(l_state);

    let _ = run_code(l_state, pretty_print);

    l_state
  }
}
