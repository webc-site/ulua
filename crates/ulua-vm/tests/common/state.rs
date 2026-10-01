//! 独立 VM 状态的 RAII 守卫：测试退出即 `lua_close`（断言失败也不漏 VM）。
//!
//! 以 `#[path = "common/state.rs"] mod state;` 被各测试 binary 单独编进自己的
//! crate（load_malformed / vm_execute / vm_contract / strtable_intern /
//! strlib_match / lua_tolstring / vm_protected_call / vm_table_rehash /
//! vm_int64_ops 共用，消掉原先各自手写的 `new`/`Drop` 样板）。
//! 各 binary 的专属辅助方法以「同 crate 内固有 impl」挂在本类型上（如
//! vm_execute 的 `pcall_ok`、vm_contract 的 `intern`）；自带夹具结构的用例
//! （vm_int64_ops 的 `Lib`、vm_table_rehash 默认分配器路径）以本守卫作字段或
//! 局部变量，不另写 `lua_close`。只有需要自定义分配器刷钩子的
//! vm_table_rehash `ProbeGuard` 走 `lua_newstate`，其关闭必须与探针记账同命，
//! 故保持独立。`luau_load` 装载入口在 `common/mod.rs`（仅前两者使用，放这里
//! 会在 vm_contract binary 里报 dead_code）。

use ulua_vm::{
  functions::{lua_close::lua_close, lua_l_newstate::lua_l_newstate},
  records::lua_state::LuaState,
};

/// 独立 VM 状态的 RAII 守卫：测试退出即 `lua_close`。
pub struct State {
  pub l: *mut LuaState,
}

impl State {
  pub fn new() -> Self {
    let l = lua_l_newstate();
    assert!(!l.is_null(), "lua_l_newstate 失败");
    Self { l }
  }
}

impl Drop for State {
  fn drop(&mut self) {
    // Safety: `self.l` 由 State::new 断言非空后独占持有，drop 即关闭 VM。
    unsafe { lua_close(self.l) };
  }
}
