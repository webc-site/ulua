use core::ffi::c_void;

use crate::{
  functions::lua_newuserdatatagged::lua_newuserdatatagged, records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活的 `LuaState` 且栈顶已预留 1 槽；返回值指向新建 tag=0 udata 的
/// `s` 字节负载（8 字节对齐），寿命随该 udata 直至被 GC 回收。
#[inline]
pub unsafe fn lua_newuserdata(l: *mut LuaState, s: usize) -> *mut c_void {
  // Safety: 契约保证 `l` 存活，tag=0 转发 lua_newuserdatatagged 建 udata 并压栈
  unsafe { lua_newuserdatatagged(l, s, 0) }
}
