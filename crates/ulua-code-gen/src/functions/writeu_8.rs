use core::mem::size_of;
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn writeu_8(target: *mut u8, value: u8) -> *mut u8 {
  unsafe {
    *target = value;
    target.add(size_of::<u8>())
  }
}
