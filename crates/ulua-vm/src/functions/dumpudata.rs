use core::ffi::{c_char, c_int, c_void};

use crate::{
  functions::dumpref::dumpref,
  macros::sizeudata::sizeudata,
  records::{gc_object::GCObject, udata::Udata},
};

pub(crate) unsafe fn dumpudata(f: *mut c_void, u: *mut Udata) {
  unsafe {
    let u = &*u;
    let fmt = "{\"type\":\"userdata\",\"cat\":%d,\"size\":%d,\"tag\":%d";

    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    }

    fprintf(
      f,
      fmt.as_ptr() as *const c_char,
      u.memcat as c_int,
      sizeudata(u.len as usize) as c_int,
      u.tag as c_int,
    );

    if !u.metatable.is_null() {
      fprintf(f, b",\"metatable\":".as_ptr() as *const c_char);
      let mt = u.metatable as *mut GCObject;
      dumpref(f, mt);
    }

    fprintf(f, b"}".as_ptr() as *const c_char);
  }
}
