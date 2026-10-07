use core::ffi::c_void;

use crate::{functions::c_file_write, records::gc_object::GCObject};

/// # Safety
/// `o` 须为存活 GCObject 的地址（仅格式化打印地址，不解引用）。
pub(crate) unsafe fn dumpref(f: *mut c_void, o: *const GCObject) {
  unsafe {
    c_file_write(f, format_args!("\"{o:p}\""));
  }
}
