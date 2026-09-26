use core::ffi::c_void;

use crate::records::lua_state::LuaState;

#[inline]
/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe fn lua_getthreaddata(l: *mut LuaState) -> *mut c_void {
  // Safety: 契约保证 `l` 指向存活协程状态，userdata 字段在其生命周期内稳定可读
  unsafe { (*l).userdata }
}
