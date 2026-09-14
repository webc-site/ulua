use core::ffi::{c_char, c_int, c_void};

use crate::{
  functions::{dumpref::dumpref, dumprefs::dumprefs},
  macros::{
    getstr::getstr, obj_2_gco::obj2gco, size_cclosure::size_cclosure, size_lclosure::size_lclosure,
  },
  records::closure::Closure,
};

pub(crate) unsafe fn dumpclosure(f: *mut c_void, cl: *mut Closure) {
  unsafe {
    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    }

    fprintf(
      f,
      c"{\"type\":\"function\",\"cat\":%d,\"size\":%d".as_ptr(),
      (*cl).hdr.memcat as c_int,
      if (*cl).is_c != 0 {
        size_cclosure((*cl).nupvalues as c_int) as c_int
      } else {
        size_lclosure((*cl).nupvalues as usize) as c_int
      },
    );

    fprintf(f, c",\"env\":".as_ptr());
    dumpref(f, obj2gco!((*cl).env));

    if (*cl).is_c != 0 {
      let c = &(*cl).inner.c;
      if !c.debugname.is_null() {
        fprintf(f, c",\"name\":\"%s\"".as_ptr(), c.debugname);
      }
      if (*cl).nupvalues != 0 {
        fprintf(f, c",\"upvalues\":[".as_ptr());
        dumprefs(f, c.upvals.as_ptr() as *mut _, (*cl).nupvalues as usize);
        fprintf(f, c"]".as_ptr());
      }
    } else {
      let l = &(*cl).inner.l;
      if !(*l.p).debugname.is_null() {
        fprintf(f, c",\"name\":\"%s\"".as_ptr(), getstr((*l.p).debugname));
      }
      fprintf(f, c",\"proto\":".as_ptr());
      dumpref(f, obj2gco!(l.p));
      if (*cl).nupvalues != 0 {
        fprintf(f, c",\"upvalues\":[".as_ptr());
        dumprefs(f, l.uprefs.as_ptr() as *mut _, (*cl).nupvalues as usize);
        fprintf(f, c"]".as_ptr());
      }
    }

    fprintf(f, c"}".as_ptr());
  }
}
