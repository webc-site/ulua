use crate::records::string_ref::StringRef;

/// Hashes a byte slice using Luau / Lua 5.1's short-string hash algorithm.
#[inline]
pub fn bytecode_builder_get_string_hash_slice(bytes: &[u8]) -> u32 {
  let mut h: u32 = bytes.len() as u32;
  for &b in bytes.iter().rev() {
    let ch = b as u32;
    h ^= (h << 5).wrapping_add(h >> 2).wrapping_add(ch);
  }
  h
}

pub fn bytecode_builder_get_string_hash(key: StringRef) -> u32 {
  // Keep in sync with Lua 5.1's original hashing algorithm:
  // https://github.com/lua/lua/blob/v5.1.5/lstrlib.c (lua_s_hash for short inputs)
  //
  // We intentionally omit long string processing for simplicity/independence
  // (matching the source logic).
  bytecode_builder_get_string_hash_slice(key.as_bytes())
}
