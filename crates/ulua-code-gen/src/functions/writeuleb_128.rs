/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn writeuleb_128(mut target: *mut u8, mut value: u64) -> *mut u8 {
  unsafe {
    loop {
      let mut byte = (value & 0x7f) as u8;
      value >>= 7;

      if value != 0 {
        byte |= 0x80;
      }

      *target = byte;
      target = target.add(1);

      if value == 0 {
        break;
      }
    }

    target
  }
}
