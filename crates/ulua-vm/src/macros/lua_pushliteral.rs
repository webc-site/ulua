use crate::{functions::lua_pushlstring::lua_pushlstring, records::lua_state::LuaState};

/// cpp `lua.h:517` `#define lua_pushliteral(L, s) lua_pushlstring(L, "" s, len)` 对应：
/// 长度即切片字节数，直接转发 `lua_pushlstring`（其 C 形态签名属 capi 导出契约，
/// 在此以切片指针收口），无返回值。
///
/// # Safety
///
/// `l` 必须指向存活的 `LuaState`。
#[inline(always)]
pub unsafe fn lua_pushliteral(l: *mut LuaState, s: &[u8]) {
  // Safety: 契约保证 `l` 存活可承接压栈；`lua_pushlstring` 仅在本次调用内读取该切片
  unsafe { lua_pushlstring(l, s.as_ptr().cast(), s.len()) }
}
