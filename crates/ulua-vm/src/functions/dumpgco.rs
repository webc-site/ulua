use core::ffi::c_void;

use crate::{
  functions::{c_file_write_bytes, dumpobj::dumpobj, dumpref::dumpref},
  records::{gc_object::GCObject, lua_page::lua_Page},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn dumpgco(
  context: *mut c_void,
  _page: *mut lua_Page,
  gco: *mut GCObject,
) -> bool {
  unsafe {
    let f = context;

    dumpref(f, gco);
    c_file_write_bytes(f, b":");
    dumpobj(f, gco);
    c_file_write_bytes(f, b",\n");

    false
  }
}
