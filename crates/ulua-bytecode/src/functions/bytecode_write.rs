use std::vec::Vec;

/// 把原始字节追加进字节缓冲（cpp `std::string::push_back(char)` 的字节语义）。
#[inline]
pub(crate) fn write_byte(ss: &mut Vec<u8>, value: u8) {
  ss.push(value);
}

/// 把原始字节追加进字节缓冲（对齐 C++ writeInt/writeFloat/writeDouble 的 memcpy）。
#[inline]
pub(crate) fn write_bytes(ss: &mut Vec<u8>, bytes: &[u8]) {
  ss.extend_from_slice(bytes);
}

/// 写入变长无符号整数编码。
pub(crate) fn write_var_int(ss: &mut Vec<u8>, mut value: u64) {
  loop {
    write_byte(ss, (value & 127) as u8 | (((value > 127) as u8) << 7));
    value >>= 7;
    if value == 0 {
      break;
    }
  }
}
