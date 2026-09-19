//! `luaL_newstate` 返回状态的 RAII 守卫：离开作用域时 `lua_close`。
//!
//! cpp 各 CLI 用 `std::unique_ptr<lua_State, void (*)(lua_State*)>` 承担同一
//! 职责；本 crate 的多个 CLI 二进制共用，故收敛为一份。

use ulua_vm::{functions::lua_close::lua_close, type_aliases::lua_state::lua_State};

/// 持有 [`lua_l_newstate`] 返回的状态指针，`Drop` 时关闭。
pub struct LuaStateGuard(pub *mut lua_State);

impl Drop for LuaStateGuard {
  fn drop(&mut self) {
    if !self.0.is_null() {
      // SAFETY: 非 null 时只可能来自 `lua_l_newstate`，且未被别处关闭。
      unsafe { lua_close(self.0) };
    }
  }
}
