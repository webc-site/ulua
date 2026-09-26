//! ReplFixture / ReplWithPathFixture 公共状态搭建（对齐 cpp 两个 fixture 构造
//! 函数相同的 luaL_newstate → setupState → luaL_sandboxthread → runCode
//! (prettyPrintSource) 序列，见 `cpp/tests/Repl.test.cpp:34-42`）。

use alloc::string::String;

use ulua_repl_cli::functions::{run_code::run_code, setup_state::setup_state};
use ulua_vm::{
  functions::{
    lua_close::lua_close, lua_l_newstate::lua_l_newstate, lua_l_sandboxthread::lua_l_sandboxthread,
  },
  macros::lua_memerrmsg::LUA_MEMERRMSG,
  records::lua_state::LuaState,
};

/// 创建沙箱化状态并执行 `pretty_print` 源码，成功时返回主线程状态。
///
/// cpp 侧 fixture 构造函数既不判空也丢弃 `runCode` 的结果；这里把两类 setup
/// 失败显式上抛，由 fixture 构造方转成一次带原因的测试失败：
/// - `luaL_newstate` 返回 null（内存耗尽）：继续 `setup_state`/`luaL_sandboxthread`
///   即对 null 状态解引用；
/// - `pretty_print` 片段编译/运行失败：静默继续会让后续断言以难以定位的方式失败。
///
/// 失败路径自行 `lua_close` 回收已建状态，调用方拿不到悬空/半初始化的句柄。
///
/// # Safety
/// 成功返回的 `LuaState` 由调用方（fixture）负责 `lua_close`；`pretty_print`
/// 必须是合法 Lua 源码。
pub(crate) unsafe fn new_fixture_state(pretty_print: &str) -> Result<*mut LuaState, String> {
  // `lua_l_newstate` 是 safe 包装（内部收口 C 分配器边界）；null 返回值在下一句
  // 判出并早退，其后所有 unsafe 调用只作用于非空状态。
  let l_state = lua_l_newstate();
  if l_state.is_null() {
    return Err(String::from_utf8_lossy(LUA_MEMERRMSG).into_owned());
  }

  // Safety: `l_state` 是上方已判非空的本帧独占活跃状态机；setup_state 完成
  // openlibs + sandbox（repl-cli 契约）。
  unsafe { setup_state(l_state) };

  // new thread needs to have the globals sandboxed
  // Safety: 同上，冻结线程全局表。
  unsafe { lua_l_sandboxthread(l_state) };

  // Safety: `run_code` 前置（活跃状态机、已 openlibs/sandbox/sandboxthread）恰由
  // 上两句成立；`pretty_print` 为合法 Lua 源码系本函数 /// # Safety 契约。
  if let Some(error) = unsafe { run_code(l_state, pretty_print) } {
    // Safety: 失败路径关闭后句柄不再被使用，调用方拿不到悬空/半初始化状态。
    unsafe { lua_close(l_state) };
    return Err(error);
  }

  Ok(l_state)
}
