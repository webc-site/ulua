use core::ffi::{c_int, c_void};

use crate::{
  functions::{
    c_file_write, c_file_write_bytes, c_file_write_str, dumpref::dumpref, dumprefs::dumprefs,
  },
  macros::{
    getstr::getstr, obj_2_gco::obj2gco, size_cclosure::size_cclosure, size_lclosure::size_lclosure,
  },
  records::closure::Closure,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn dumpclosure(f: *mut c_void, cl: *mut Closure) {
  unsafe {
    c_file_write(
      f,
      format_args!(
        "{{\"type\":\"function\",\"cat\":{},\"size\":{}",
        (*cl).hdr.memcat,
        if (*cl).is_c != 0 {
          size_cclosure((*cl).nupvalues as c_int) as c_int
        } else {
          size_lclosure((*cl).nupvalues as usize) as c_int
        },
      ),
    );

    c_file_write_bytes(f, b",\"env\":");
    dumpref(f, obj2gco!((*cl).env));

    if (*cl).is_c != 0 {
      let c = &(*cl).inner.c;
      if !c.debugname.is_null() {
        c_file_write_bytes(f, b",\"name\":\"");
        c_file_write_str(f, c.debugname);
        c_file_write_bytes(f, b"\"");
      }
      if (*cl).nupvalues != 0 {
        c_file_write_bytes(f, b",\"upvalues\":[");
        dumprefs(f, c.upvals.as_ptr() as *mut _, (*cl).nupvalues as usize);
        c_file_write_bytes(f, b"]");
      }
    } else {
      let l = &(*cl).inner.l;
      if !(*l.p).debugname.is_null() {
        c_file_write_bytes(f, b",\"name\":\"");
        c_file_write_str(f, getstr((*l.p).debugname));
        c_file_write_bytes(f, b"\"");
      }
      c_file_write_bytes(f, b",\"proto\":");
      dumpref(f, obj2gco!(l.p));
      if (*cl).nupvalues != 0 {
        c_file_write_bytes(f, b",\"upvalues\":[");
        dumprefs(f, l.uprefs.as_ptr() as *mut _, (*cl).nupvalues as usize);
        c_file_write_bytes(f, b"]");
      }
    }

    c_file_write_bytes(f, b"}");
  }
}
