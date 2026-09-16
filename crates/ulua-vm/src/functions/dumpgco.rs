use core::ffi::c_void;

use crate::{
  functions::{c_file_write_bytes, dumpobj::dumpobj, dumpref::dumpref},
  records::{gc_object::GCObject, lua_page::lua_Page},
};

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
