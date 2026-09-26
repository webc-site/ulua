use ulua_vm::{
  functions::{
    lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
    lua_l_sandboxthread::lua_l_sandboxthread,
  },
  records::lua_state::LuaState,
};

/// cpp `tests/Conformance.test.cpp` 各手工用例里逐字重复的建库前置：
/// `luaL_openlibs(L); luaL_sandbox(L); luaL_sandboxthread(L);`（如
/// `Conformance.test.cpp:4452-4454`、`DirectFieldAccess` 与 huge-codegen 系列）。
///
/// 三步恒以同一顺序、同一 `LuaState` 连用，中间不夹杂其它调用，是「直接调 C API
/// 的用例」统一的库打开 + 沙箱化入口。收敛前本 crate 内有 8 份同形副本
/// （`bytecode.rs` 4 份、`codegen.rs` 3 份、`debugger.rs` 1 份），现全部切到本门面。
///
/// # Safety
/// 调用方须保证 `l` 是存活、尚未 openlibs 的 `LuaState`（上游同一前提）：本函数
/// 会往该状态的标准库里注册全局并沙箱化，重复对同一状态调用会二次注册。
pub unsafe fn openlibs_and_sandbox(l: *mut LuaState) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 由调用方在本用例作用域内取得（`new_state`
  // / `lua_newstate` 后 `as_ptr`），至本函数全程存活，满足三个被调 unsafe 例程的
  // LuaState 前置条件。
  unsafe {
    lua_l_openlibs(l);
    lua_l_sandbox(l);
    lua_l_sandboxthread(l);
  }
}
