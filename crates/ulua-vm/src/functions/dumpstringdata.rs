use core::{
  ffi::{c_char, c_int, c_void},
  slice::from_raw_parts,
};

use crate::functions::safejson::safejson;

pub(crate) unsafe fn dumpstringdata(f: *mut c_void, data: *const c_char, len: usize) {
  unsafe {
    let slice = from_raw_parts(data, len);
    for &ch in slice {
      let out = if safejson(ch) { ch } else { '?' as c_char };

      // Note: fputc is provided by the system's C library.
      // In this crate's context, we call it via the extern "C" linkage.
      unsafe extern "C" {
        fn fputc(c: c_int, stream: *mut c_void) -> c_int;
      }

      fputc(out as c_int, f);
    }
  }
}
