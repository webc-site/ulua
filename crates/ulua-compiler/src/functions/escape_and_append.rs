use alloc::vec::Vec;

/// Escapes '%' characters by duplicating them and appends the resulting bytes to `buffer`.
pub(crate) fn escape_and_append(buffer: &mut Vec<u8>, s: &[u8]) {
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
