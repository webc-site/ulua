//! 独立 VM 状态的 RAII 守卫：测试退出即 `lua_close`。
//!
//! 以 `#[path = "common/state.rs"] mod state;` 被各测试 binary 单独编进自己的
//! crate（load_malformed / vm_execute / vm_contract 三处共用，消掉原先各自
//! 手写的 `new`/`Drop` 样板）。各 binary 的专属辅助方法以「同 crate 内固有
//! impl」挂在本类型上（如 vm_execute 的 `pcall_ok`、vm_contract 的 `intern`）；
//! `luau_load` 装载入口在 `common/mod.rs`（仅前两者使用，放这里会在
//! vm_contract binary 里报 dead_code）。

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
