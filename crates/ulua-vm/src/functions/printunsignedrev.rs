use core::ffi::c_char;

/// `00`–`99` 两位十进制数字查找表（cpp lnumprint.cpp `kDigitTable`）。
/// `printunsignedrev` 与 `printexp` 共用，避免重复的同款字符串字面量。
pub(crate) const K_DIGIT_TABLE: [u8; 200] = *b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899";

/// 向 `dst`（恰 2 字节）写 `v`（< 100）对应的两位十进制数字。
#[inline]
fn write2(dst: &mut [c_char], v: u32) {
  let digits = &K_DIGIT_TABLE[(v as usize) * 2..(v as usize) * 2 + 2];
  dst[0] = digits[0] as c_char;
  dst[1] = digits[1] as c_char;
}

/// cpp `lnumprint.cpp:printunsignedrev`：把 `num` 的十进制数字自 `buf` 末端
/// 向左回写（右对齐），返回首位数字的下标。
///
/// 前置条件：`buf.len()` 足以容纳 `num` 的全部十进制位数。
pub(crate) fn printunsignedrev(buf: &mut [c_char], mut num: u64) -> usize {
  let mut end = buf.len();

  while num >= 10000 {
    let tail = (num % 10000) as u32;
    write2(&mut buf[end - 4..end - 2], tail / 100);
    write2(&mut buf[end - 2..end], tail % 100);
    num /= 10000;
    end -= 4;
  }

  let mut rest = num as u32;

  while rest >= 10 {
    write2(&mut buf[end - 2..end], rest % 100);
    rest /= 100;
    end -= 2;
  }

  if rest > 0 {
    buf[end - 1] = (b'0' + rest as u8) as c_char;
    end -= 1;
  }

  end
}
