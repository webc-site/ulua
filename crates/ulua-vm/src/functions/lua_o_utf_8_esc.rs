use core::ffi::c_char;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::macros::utf_8_buffsz::UTF8BUFFSZ;

/// Encodes a Unicode code point into a UTF-8 byte sequence stored in the
/// buffer, starting from the end. Returns the number of bytes used.
pub fn lua_o_utf_8_esc(buff: &mut [c_char; UTF8BUFFSZ], mut x: u32) -> i32 {
  let mut n: i32 = 1; // number of bytes put in buffer (backwards)
  LUAU_ASSERT!(x <= 0x10FFFF);
  if x < 0x80 {
    // ascii?
    buff[UTF8BUFFSZ - 1] = x as c_char;
  } else {
    // need continuation bytes
    let mut mfb: u32 = 0x3f; // maximum that fits in first byte
    loop {
      // add continuation bytes
      buff[UTF8BUFFSZ - (n as usize)] = (0x80 | (x & 0x3f)) as c_char;
      n += 1;
      x >>= 6; // remove added bits
      mfb >>= 1; // now there is one less bit available in first byte
      if !(x > mfb) {
        break;
      }
    }
    buff[UTF8BUFFSZ - (n as usize)] = ((!mfb << 1) | x) as c_char;
    // add first byte
  }
  n
}
