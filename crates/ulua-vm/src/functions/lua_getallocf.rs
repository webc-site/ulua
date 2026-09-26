use core::ffi::c_void;

use crate::{records::lua_state::LuaState, type_aliases::lua_alloc::LuaAlloc};

/// # Safety
///
/// `l` 必须指向存活的 `LuaState`；`ud` 若非空必须指向可写的 `*mut c_void` 槽。
pub unsafe fn lua_getallocf(l: *mut LuaState, ud: *mut *mut c_void) -> LuaAlloc {
  // Safety: 契约保证 `l` 指向存活 LuaState，global 链与 frealloc 字段在状态生命周期内稳定可读
  let f = unsafe { (*(*l).global).frealloc };
  if !ud.is_null() {
    // Safety: 同上 `l` 存活；ud 非空已由外层检查，写回仅拷贝用户指针值
    unsafe { *ud = (*(*l).global).ud };
  }
  f
}
