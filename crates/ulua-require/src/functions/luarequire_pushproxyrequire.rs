use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{lua_proxyrequire::lua_proxyrequire, push_closure::push_closure},
  records::navigation_context::RequireHost,
};

/// proxyrequire 闭包的调试名（NUL 结尾静态字节串）。
const PROXY_REQUIRE_DEBUGNAME: &[u8] = b"proxyrequire\0";

/// 建立并压入 proxyrequire 闭包（cpp `luarequire_pushproxyrequire`）：以
/// `(path, requirerChunkname)` 两参按既有模块视角解析路径。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`（宿主 Lua/C API 句柄，本 crate 归 ulua-vm
/// C-API 真边界裁定）；`host` 前提同 [`crate::functions::luarequire_pushrequire`]。
pub unsafe fn luarequire_pushproxyrequire<C: RequireHost + 'static>(
  l: *mut LuaState,
  host: C,
) -> i32 {
  // Safety: l 为宿主提供的有效 LuaState；host 经 push_closure 装箱移交；
  // `lua_proxyrequire::<C>` 是本 crate 静态存活闭包体按同一宿主类型 `C` 的
  // 单态化实例（coerce 为 C 函数指针）。
  unsafe {
    push_closure(
      l,
      host,
      Some(lua_proxyrequire::<C>),
      PROXY_REQUIRE_DEBUGNAME,
    )
  }
}
