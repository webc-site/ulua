use core::ffi::{c_int, c_void};

use crate::{
  functions::{dumpobj::dumpobj, dumpref::dumpref},
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
    unsafe extern "C" {
      fn fputc(c: c_int, stream: *mut c_void) -> c_int;
    }
    fputc(':' as c_int, f);
    dumpobj(f, gco);
    fputc(',' as c_int, f);
    fputc('\n' as c_int, f);

    false
  }
}
