use alloc::string::String;

use crate::functions::write_bytes::write_bytes;

/// C++ `writeDouble`：按本机字节序追加 8 字节。
pub(crate) fn write_double(ss: &mut String, value: f64) {
  write_bytes(ss, &value.to_ne_bytes());
}
