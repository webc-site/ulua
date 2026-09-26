use core::ffi::c_void;

use crate::{
  functions::enumobj::enumobj,
  records::{enum_context::EnumContext, gc_object::GCObject, lua_page::lua_Page},
};

/// # Safety
/// `context` 必须能被还原为存活的 `EnumContext*`（由 `lua_enumgcobjects` 系列入口传入）；`gco` 须为页
/// 遍历回调时刻仍存活的 GCObject，`enumobj` 会按其类型标签读取内部字段。违反则对已回收对象做类型分派。
/// cpp lgcdebug.cpp:1122。
pub(crate) unsafe fn enumgco(
  context: *mut c_void,
  _page: *mut lua_Page,
  gco: *mut GCObject,
) -> bool {
  // Safety: 契约保证 `gco` 为存活 GCObject，按 LuaType 分派 enumedges/枚举串的遍历前置均成立
  unsafe {
    let enum_ctx = context as *mut EnumContext;

    enumobj(enum_ctx, gco);

    false
  }
}
