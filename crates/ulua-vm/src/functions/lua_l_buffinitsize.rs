use core::ffi::c_char;

use crate::{
  functions::{lua_l_buffinit::lua_l_buffinit, lua_l_prepbuffsize::lua_l_prepbuffsize},
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// # Safety
/// `b`/`L` 必须相互一致且 `b` 处于 buffinitsize 之后、pushresultsize 之前的有效状态；size 为本次需要的字节数。
pub(crate) unsafe fn lua_l_buffinitsize(
  l: *mut LuaState,
  b: *mut LuaLStrbuf,
  size: usize,
) -> *mut c_char {
  // Safety: 契约保证 `L`/`b` 一致且 size 经溢出防护，块内分配的缓冲挂回栈值并同步 L->top 引用
  unsafe {
    lua_l_buffinit(l, b);
    lua_l_prepbuffsize(b, size)
  }
}
