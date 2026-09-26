use crate::{
  functions::lua_s_newlstr::lua_s_newlstr,
  records::{lua_state::LuaState, t_string::tstring},
};

/// C++ `lua_s_new` 宏（lstring.h）对应：长度由字节切片携带，走 `lua_s_newlstr` 驻留。
/// 与 `lua_s_newliteral`（lstring.h 的另一宏镜像）实现相同，后者保留于独立模块。
///
/// # Safety
///
/// `l` 必须指向存活的 `LuaState`。
pub unsafe fn lua_s_new(l: *mut LuaState, s: &[u8]) -> *mut tstring {
  // Safety: 契约保证 `l` 存活可完成字符串驻留
  unsafe { lua_s_newlstr(l, s) }
}
