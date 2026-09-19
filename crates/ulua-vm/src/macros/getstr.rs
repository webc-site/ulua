use core::ffi::c_char;

use crate::records::t_string::tstring;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn getstr(ts: *const tstring) -> *const c_char {
  unsafe { (*ts).data.as_ptr() }
}
