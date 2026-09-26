/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn writeuleb_128(mut target: *mut u8, mut value: u64) -> *mut u8 {
  // Safety: 契约保证 target 为存活可写字节缓冲且预留了 LEB128 最坏 10 字节空间；
  // value 每轮右移 7 位收敛，循环至多发射 10 字节，target.add(1) 不越出该预留区，
  // u8 写对齐要求平凡满足。下文各轮窄块统一简记「依契约」。
  loop {
    let mut byte = (value & 0x7f) as u8;
    value >>= 7;

    if value != 0 {
      byte |= 0x80;
    }

    // Safety: 依契约——在预留区内写 1 字节并前进一位（u8 对齐平凡）。
    unsafe {
      *target = byte;
      target = target.add(1);
    }

    if value == 0 {
      break;
    }
  }

  target
}
