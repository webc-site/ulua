use core::ffi::c_void;

use crate::{functions::c_file_write, records::gc_object::GCObject};

pub(crate) unsafe fn dumpref(f: *mut c_void, o: *mut GCObject) {
  unsafe {
    c_file_write(f, format_args!("\"{o:p}\""));
  }
}
