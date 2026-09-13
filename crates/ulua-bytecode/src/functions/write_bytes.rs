use alloc::string::String;

/// 把原始字节追加进 bytecode 缓冲。
/// SAFETY：bytecode 的 String 是 C++ `std::string` 字节容器的等价物，
/// 全程不维护 UTF-8 不变量（与 C++ writeInt/writeFloat/writeDouble 的 memcpy 对齐）。
pub(crate) fn write_bytes(ss: &mut String, bytes: &[u8]) {
  unsafe {
    ss.as_mut_vec().extend_from_slice(bytes);
  }
}
