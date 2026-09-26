use core::ffi::c_void;

use crate::{
  functions::validateobj::validateobj,
  records::{
    gc_object::GCObject, global_state::global_State, lua_page::lua_Page, lua_state::LuaState,
  },
};

/// # Safety
/// 输入字节流缓冲与读游标必须位于本次加载的串数据界内，长度参数覆盖所有被读字段。
pub(crate) unsafe fn validategco(
  context: *mut c_void,
  _page: *mut lua_Page,
  gco: *mut GCObject,
) -> bool {
  // Safety: 契约保证 `gco` 为存活 GCObject，按类型分派 traverseproto/traverseobject 的引用遍历仅在对象界内
  unsafe {
    let l = context as *mut LuaState;
    let g: *mut global_State = (*l).global;

    validateobj(g, gco);

    false
  }
}
