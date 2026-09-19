use alloc::vec::Vec;

/// 把原始字节追加进字节缓冲（对齐 C++ writeInt/writeFloat/writeDouble 的 memcpy）。
pub(crate) fn write_bytes(ss: &mut Vec<u8>, bytes: &[u8]) {
  ss.extend_from_slice(bytes);
}
