use core::{
  ffi::{c_char, c_int, c_void},
  mem::size_of,
};

use crate::{
  functions::dumpref::dumpref,
  macros::{gcvalue::gcvalue, iscollectable::iscollectable, upisopen::upisopen},
  type_aliases::up_val::UpVal,
};

pub(crate) unsafe fn dumpupval(f: *mut c_void, uv: *mut UpVal) {
  unsafe {
    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    }

    let uv_ref = &*uv;

    let is_open = upisopen!(uv);

    fprintf(
      f,
      c"{\"type\":\"upvalue\",\"cat\":%d,\"size\":%d,\"open\":%s".as_ptr() as *const c_char,
      uv_ref.hdr.memcat as c_int,
      size_of::<UpVal>() as c_int,
      if is_open {
        c"true".as_ptr()
      } else {
        c"false".as_ptr()
      } as *const c_char,
    );

    if iscollectable!(uv_ref.v) {
      fprintf(f, c",\"object\":".as_ptr() as *const c_char);
      dumpref(f, gcvalue!(uv_ref.v));
    }

    fprintf(f, c"}".as_ptr() as *const c_char);
  }
}
