use core::ffi::c_void;

use crate::{functions::c_file_write, records::gc_object::GCObject};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn dumpref(f: *mut c_void, o: *mut GCObject) {
  unsafe {
    c_file_write(f, format_args!("\"{o:p}\""));
  }
}
