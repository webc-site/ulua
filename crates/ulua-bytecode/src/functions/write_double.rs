use std::vec::Vec;

use crate::functions::write_bytes::write_bytes;

/// C++ `writeDouble`：按本机字节序追加 8 字节。
pub(crate) fn write_double(ss: &mut Vec<u8>, value: f64) {
  write_bytes(ss, &value.to_ne_bytes());
}
