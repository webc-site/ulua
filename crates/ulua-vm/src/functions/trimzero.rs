use core::ffi::c_char;
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub(crate) unsafe fn trimzero(mut end: *mut c_char) -> *mut c_char {
  unsafe {
    while *end.offset(-1) == b'0' as c_char {
      end = end.offset(-1);
    }

    end
  }
}
