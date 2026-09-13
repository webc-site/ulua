use alloc::vec::Vec;

pub fn from_utf_8(path: &str) -> Vec<u16> {
  path.encode_utf16().collect()
}
