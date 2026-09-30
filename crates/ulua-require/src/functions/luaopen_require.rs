use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::luarequire_pushrequire::luarequire_pushrequire,
  records::navigation_context::RequireHost,
};

/// 全局表注册名（`LuaState::set_global_str` 收口点使用）。
pub(crate) const REQUIRE_GLOBAL: &str = "require";

/// 初始化 require 库并注册到全局表（cpp `luaopen_require`）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`（宿主 Lua/C API 句柄，本 crate 归 ulua-vm
/// C-API 真边界裁定）；`host` 前提同 [`luarequire_pushrequire`]。
pub unsafe fn luaopen_require<C: RequireHost + 'static>(l: *mut LuaState, host: C) {
  // Safety: l 为宿主开启 require 库时提供的存活 LuaState；pushrequire 按
  // 自身契约装箱宿主并挂闭包，set_global_str 消费栈顶闭包（净变化 0）。
  unsafe {
    luarequire_pushrequire(l, host);
    (*l).set_global_str(REQUIRE_GLOBAL);
  }
}
