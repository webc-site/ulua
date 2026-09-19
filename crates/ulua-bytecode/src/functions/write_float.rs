use std::vec::Vec;

use crate::functions::write_bytes::write_bytes;

/// C++ `writeFloat`：按本机字节序追加 4 字节。
pub(crate) fn write_float(ss: &mut Vec<u8>, value: f32) {
  write_bytes(ss, &value.to_ne_bytes());
}
