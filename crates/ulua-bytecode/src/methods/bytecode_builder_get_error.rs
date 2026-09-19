use std::vec::Vec;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  /// 错误字节码 blob：首字节 0 为错误标记（等同合法 blob 的 LBC_VERSION_TARGET），
  /// 其后为可读消息的原始字节。
  pub fn get_error(message: &str) -> Vec<u8> {
    // 0 acts as a special marker for error bytecode (it's equal to LBC_VERSION_TARGET for valid bytecode blobs)
    let mut result = Vec::with_capacity(message.len() + 1);
    result.push(0u8);
    result.extend_from_slice(message.as_bytes());

    result
  }
}
