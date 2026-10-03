//! `luau_temp_thread_popper` 的 Rust 形态：记录临时线程弹出前需归还的父 state。
//!
//! cpp 侧该 RAII 守卫只存一枚 `lua_State*`，析构时 `lua_pop(L, 1)`。本仓库对
//! 「只作身份传递、不就地解引用」的 VM 句柄统一保留裸指针形态（同 `StateRef`），
//! 解引用一律收口到 `arena_handle::alias` 门面一处。

use ulua_vm::records::lua_state::LuaState;

#[derive(Debug, Clone)]
pub struct LuauTempThreadPopper {
  /// 身份句柄：仅记录待弹出其栈槽的父 state 地址，存活期由宿主 `StateRef` 保证，
  /// 本记录不解引用（弹出经 `luau_temp_thread_popper` 内的 `alias` 收口）。
  pub(crate) l: *mut LuaState,
}
