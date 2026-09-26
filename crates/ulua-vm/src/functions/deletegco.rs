use core::ffi::c_void;

use crate::{
  functions::freeobj::freeobj,
  records::{gc_object::GCObject, lua_page::lua_Page, lua_state::LuaState},
};

/// # Safety
/// `context` 须为页分配器遍历回调给出的存活 LuaState 指针，`gco` 须为该 `page` 上待回收对象且 `page` 元数据与其一致。
pub(crate) unsafe fn deletegco(
  context: *mut c_void,
  page: *mut lua_Page,
  gco: *mut GCObject,
) -> bool {
  // Safety: 契约保证 context/page/gco 来自页分配器遍历回调且相互一致（gco 为该页上待回收对象），内部转发 freeobj 释放
  unsafe {
    let l = context as *mut LuaState;
    freeobj(l, gco, page);
    true
  }
}
