use core::ffi::c_void;

use crate::{
  functions::{c_file_write_bytes, dumpobj::dumpobj, dumpref::dumpref},
  records::{gc_object::GCObject, lua_page::lua_Page},
};

/// # Safety
/// `context` 须为可写的输出目标（FILE* 转 c_void，dump 全程打开）；`gco` 须为页遍历回调时刻仍存活、
/// 类型标签可读的 GCObject（`_page` 仅为签名对齐未使用）。违反则向已回收对象序列化，写出悬垂地址或 UB。
/// cpp lgcdebug.cpp:695。
pub(crate) unsafe fn dumpgco(
  context: *mut c_void,
  _page: *mut lua_Page,
  gco: *mut GCObject,
) -> bool {
  // Safety: 契约保证 `gco` 指向存活 GCObject 且类型标签可读，块内按其 LuaType 序列化不越过对象界
  unsafe {
    let f = context;

    dumpref(f, gco);
    c_file_write_bytes(f, b":");
    dumpobj(f, gco);
    c_file_write_bytes(f, b",\n");

    false
  }
}
