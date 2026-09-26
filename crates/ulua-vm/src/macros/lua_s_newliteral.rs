use crate::{
  functions::lua_s_newlstr::lua_s_newlstr,
  records::{lua_state::LuaState, t_string::tstring},
};

/// C++ `luaS_newliteral` 宏（lstring.h）对应：原 NUL 结尾字面量约定改为字节切片入参，
/// 长度即 `s.len()`（嵌入 NUL 字节不再截断，语义为显式字节串）。cpp lstring.h 的宏镜像
/// 保留于独立模块，与 `lua_s_new` 实现相同。
///
/// # Safety
///
/// `l` 必须指向存活的 `LuaState`。
pub unsafe fn lua_s_newliteral(l: *mut LuaState, s: &[u8]) -> *mut tstring {
  // Safety: 契约保证 `l` 存活可完成字符串驻留
  unsafe { lua_s_newlstr(l, s) }
}
