//! `luaL_newstate` 返回状态的 RAII 守卫：离开作用域时 `lua_close`。
//!
//! cpp 各 CLI 用 `std::unique_ptr<lua_State, void (*)(lua_State*)>` 承担同一
//! 职责；守卫的类型与语义都随 `lua_State` 走，故归属 VM crate，供 CLI 各
//! 二进制与 `ulua` 伞 crate 共用。

use crate::{functions::lua_close::lua_close, records::lua_state::LuaState};

/// 持有 [`crate::functions::lua_l_newstate`] 返回的状态指针，`Drop` 时关闭。
#[derive(Debug)]
pub struct LuaStateGuard(pub *mut LuaState);

impl Drop for LuaStateGuard {
  fn drop(&mut self) {
    if !self.0.is_null() {
      // Safety: 非 null 时只可能来自 `lua_l_newstate`，且未被别处关闭。
      unsafe { lua_close(self.0) };
    }
  }
}
