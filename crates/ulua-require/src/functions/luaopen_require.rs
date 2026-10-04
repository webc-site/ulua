use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::luarequire_pushrequire::luarequire_pushrequire,
  records::navigation_context::RequireHost,
};

/// 全局表注册名（`LuaState::set_global_str` 收口点使用）。
pub(crate) const REQUIRE_GLOBAL: &str = "require";

/// 初始化 require 库并注册到全局表（cpp `luaopen_require`）。
///
/// 收形（review.md §2/§3）：`l` 由 `*mut LuaState` 收编为独占借用 `&mut LuaState`，
/// 存活与独占前提由类型承载，故降为安全 `fn`；体内两步（压闭包、注册全局）都已是
/// 安全实现，无残余裸操作。调用点（repl-cli `setup_state`、本 crate 测试夹具）本
/// 就已物化好借用，直传引用即可，不再在边界折回裸指针。
///
/// 调用序契约（正确性，非内存安全）：`host` 须为 `C: 'static` 的宿主值，装箱进与
/// 闭包同寿命的 userdata 后由 GC 终结；调用须在受保护帧内进行（建 userdata/闭包可
/// 分配并触发 GC）；结束时栈净变化 0（闭包从栈顶移入 `_G.require`）。
pub fn luaopen_require<C: RequireHost + 'static>(l: &mut LuaState, host: C) {
  // 装箱宿主并压入 require 闭包（净压一值）。
  luarequire_pushrequire(l, host);
  // 消费栈顶闭包注册到全局表（安全方法，净变化 0）。
  l.set_global_str(REQUIRE_GLOBAL);
}
