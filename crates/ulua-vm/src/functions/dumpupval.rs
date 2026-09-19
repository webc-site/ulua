use core::{
  ffi::{c_int, c_void},
  mem::size_of,
};

use crate::{
  functions::{c_file_write, c_file_write_bytes, dumpref::dumpref},
  macros::{gcvalue::gcvalue, iscollectable::iscollectable, upisopen::upisopen},
  records::up_val::UpVal,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn dumpupval(f: *mut c_void, uv: *mut UpVal) {
  unsafe {
    let uv_ref = &*uv;

    let is_open = upisopen!(uv);

    c_file_write(
      f,
      format_args!(
        "{{\"type\":\"upvalue\",\"cat\":{},\"size\":{},\"open\":{}",
        uv_ref.hdr.memcat,
        size_of::<UpVal>() as c_int,
        if is_open { "true" } else { "false" }
      ),
    );

    if iscollectable!(uv_ref.v) {
      c_file_write_bytes(f, b",\"object\":");
      dumpref(f, gcvalue!(uv_ref.v));
    }

    c_file_write_bytes(f, b"}");
  }
}
