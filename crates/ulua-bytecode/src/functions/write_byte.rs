use alloc::vec::Vec;

/// 把原始字节追加进字节缓冲（cpp `std::string::push_back(char)` 的字节语义）。
pub(crate) fn write_byte(ss: &mut Vec<u8>, value: u8) {
  ss.push(value);
}
