use core::ffi::{c_char, c_int, c_void};

use crate::{
  functions::dumpstringdata::dumpstringdata, macros::sizestring::sizestring,
  type_aliases::t_string::tstring,
};

pub(crate) unsafe fn dumpstring(f: *mut c_void, ts: *mut tstring) {
  unsafe {
    let ts = &*ts;
    let fmt = "{\"type\":\"string\",\"cat\":%d,\"size\":%d,\"data\":\"";

    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    }

    fprintf(
      f,
      fmt.as_ptr() as *const c_char,
      ts.hdr.memcat as c_int,
      sizestring(ts.len as usize) as c_int,
    );

    dumpstringdata(f, ts.data.as_ptr(), ts.len as usize);

    fprintf(f, b"\"}\"".as_ptr() as *const c_char);
  }
}
