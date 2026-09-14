use core::ffi::{c_char, c_int, c_void};

use crate::records::gc_object::GCObject;

pub(crate) unsafe fn dumpref(f: *mut c_void, o: *mut GCObject) {
  unsafe {
    let fmt = "\"%p\"\0";

    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    }

    fprintf(f, fmt.as_ptr() as *const c_char, o);
  }
}
