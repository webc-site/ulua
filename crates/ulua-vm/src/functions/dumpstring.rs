use core::ffi::{c_int, c_void};

use crate::{
  functions::{c_file_write, c_file_write_bytes, dumpstringdata::dumpstringdata},
  macros::sizestring::sizestring,
  type_aliases::t_string::tstring,
};

pub(crate) unsafe fn dumpstring(f: *mut c_void, ts: *mut tstring) {
  unsafe {
    let ts = &*ts;

    c_file_write(
      f,
      format_args!(
        "{{\"type\":\"string\",\"cat\":{},\"size\":{},\"data\":\"",
        ts.hdr.memcat,
        sizestring(ts.len as usize) as c_int
      ),
    );

    dumpstringdata(f, ts.data.as_ptr(), ts.len as usize);

    c_file_write_bytes(f, b"\"}\"");
  }
}
