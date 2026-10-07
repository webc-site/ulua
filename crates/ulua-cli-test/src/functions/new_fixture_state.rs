//! ReplFixture / ReplWithPathFixture 公共状态搭建（对齐 cpp 两个 fixture 构造
//! 函数相同的 luaL_newstate → setupState → luaL_sandboxthread → runCode
//! (prettyPrintSource) 序列，见 `cpp/tests/Repl.test.cpp:34-42`）。

use alloc::string::String;

use ulua_repl_cli::functions::{run_code::run_code, setup_state::setup_state};
use ulua_vm::{
  functions::{lua_l_newstate::lua_l_newstate, lua_l_sandboxthread::lua_l_sandboxthread},
  macros::lua_memerrmsg::LUA_MEMERRMSG_STR,
  records::lua_state_guard::LuaStateGuard,
};

/// 创建沙箱化状态并执行 `pretty_print` 源码，成功时返回主线程状态的 RAII 守卫
/// （`Drop` 里 `lua_close`，对应 cpp `unique_ptr<LuaState, void(*)(LuaState*)>`
/// 成员）。裸指针只在函数体内收口，不外泄给夹具。
///
/// cpp 侧 fixture 构造函数既不判空也丢弃 `runCode` 的结果；这里把两类 setup
/// 失败显式上抛，由 fixture 构造方转成一次带原因的测试失败：
/// - `luaL_newstate` 返回 null（内存耗尽）：继续 `setup_state`/`luaL_sandboxthread`
///   即对 null 状态解引用；
/// - `pretty_print` 片段编译/运行失败：静默继续会让后续断言以难以定位的方式失败。
///
/// 失败路径由守卫 `Drop` 回收已建状态，调用方拿不到悬空/半初始化的句柄。
/// `pretty_print` 必须是合法 Lua 源码（夹具契约，失败仅表现为本函数返回 `Err`）。
pub(crate) fn new_fixture_state(pretty_print: &str) -> Result<LuaStateGuard, String> {
  // `lua_l_newstate` 是 safe 包装（内部收口 C 分配器边界）。
  let l_state = lua_l_newstate();
  // 真边界：`lua_l_newstate` 返回的 `*mut LuaState` 是 VM 状态裸指针契约，null 返回值
  // 在下一句判出并早退，其后所有 unsafe 调用只作用于非空状态。
  if l_state.is_null() {
    return Err(LUA_MEMERRMSG_STR.to_owned());
  }

  // 判空后立即入守卫：成功/失败两路统一由 `Drop` 收口 `lua_close`，
  // 不再有手动的失败路径 close。
  let state = LuaStateGuard(l_state);

  // Safety: `state.0` 是上方已判非空的本帧独占活跃状态机，存活期覆盖下面整段初始化
  // 与 pretty printer 执行（守卫直到本函数返回才 close）；夹具为单线程驱动，每个
  // `&mut *` 物化的借用窗都止于当句调用、窗内无并存可变别名。setup_state/sandboxthread
  // 的「刚创建、有效」前置即此契约。
  let l = unsafe { &mut *state.0 };
  // new thread needs to have the globals sandboxed
  setup_state(l);
  lua_l_sandboxthread(l);

  // `run_code` 前置（活跃状态机、已 openlibs/sandbox/sandboxthread）恰由上两句成立；
  // `pretty_print` 为合法 Lua 源码系本函数文档契约。
  if let Some(error) = run_code(l, pretty_print) {
    return Err(error);
  }

  Ok(state)
}
