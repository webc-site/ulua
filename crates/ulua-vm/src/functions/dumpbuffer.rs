use core::ffi::{c_int, c_void};

use crate::{
  functions::c_file_write, macros::sizebuffer::sizebuffer, type_aliases::buffer::Buffer,
};

pub(crate) unsafe fn dumpbuffer(f: *mut c_void, b: *mut Buffer) {
  unsafe {
    let b = &*b;
    c_file_write(
      f,
      format_args!(
        "{{\"type\":\"buffer\",\"cat\":{},\"size\":{}}}",
        b.memcat,
        sizebuffer(b.len as usize) as c_int
      ),
    );
  }
}
