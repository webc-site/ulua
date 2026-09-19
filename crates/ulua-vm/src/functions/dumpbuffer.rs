use core::ffi::{c_int, c_void};

use crate::{
  functions::c_file_write, macros::sizebuffer::sizebuffer,
  records::luau_buffer::LuauBuffer as Buffer,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
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
