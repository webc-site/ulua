use ulua_common::functions::lua51_hash_tail::lua51_hash_tail;

use crate::records::string_ref::StringRef;

/// Hashes a byte slice using Luau / Lua 5.1's short-string hash algorithm.
/// 尾循环单源在 [`lua51_hash_tail`]（与 ulua-vm `lua_s_hash` 的尾段共享同一实现，
/// 逐位一致由该单点保证）；本侧按 cpp `getStringHash` 语义省略长串分块，
/// 全长直接进尾段，种子取长度。
#[inline]
pub(crate) fn bytecode_builder_get_string_hash_slice(bytes: &[u8]) -> u32 {
  lua51_hash_tail(bytes, bytes.len() as u32)
}

pub fn bytecode_builder_get_string_hash(key: StringRef<'_>) -> u32 {
  // Keep in sync with Lua 5.1's original hashing algorithm:
  // https://github.com/lua/lua/blob/v5.1.5/lstrlib.c (lua_s_hash for short inputs)
  //
  // We intentionally omit long string processing for simplicity/independence
  // (matching the source logic).
  bytecode_builder_get_string_hash_slice(key.as_bytes())
}
