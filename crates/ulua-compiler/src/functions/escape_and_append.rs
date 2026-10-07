use alloc::vec::Vec;

use memchr::memchr;

/// 将 '%' 字符转义为 '%%' 后把结果字节追加进 `buffer`。
pub fn escape_and_append(buffer: &mut Vec<u8>, s: &[u8]) {
  // 是否含 '%' 的字节扫描交 memchr（SIMD）；builder 式 `&mut Vec<u8>` sink 保留。
  if memchr(b'%', s).is_some() {
    for &character in s {
      buffer.push(character);

      if character == b'%' {
        buffer.push(b'%');
      }
    }
  } else {
    buffer.extend_from_slice(s);
  }
}
