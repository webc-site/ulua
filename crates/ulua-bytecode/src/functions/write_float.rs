use alloc::string::String;

use crate::functions::write_bytes::write_bytes;

/// C++ `writeFloat`：按本机字节序追加 4 字节。
pub(crate) fn write_float(ss: &mut String, value: f32) {
  write_bytes(ss, &value.to_ne_bytes());
}
