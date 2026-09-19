use core::ffi::c_char;
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub(crate) unsafe fn iscont(p: *const c_char) -> bool {
  unsafe { ((*p as u8) & 0xC0) == 0x80 }
}
