use alloc::vec::Vec;

/// 将 '%' 字符转义为 '%%' 后把结果字节追加进 `buffer`。
pub fn escape_and_append(buffer: &mut Vec<u8>, s: &[u8]) {
  if s.contains(&b'%') {
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
