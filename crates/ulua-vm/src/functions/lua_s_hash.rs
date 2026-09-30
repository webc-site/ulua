use core::ffi::c_uint;

use ulua_common::functions::lua51_hash_tail::lua51_hash_tail;

use crate::macros::mix::mix;

/// 前缀哈希的分块宽度（字节）：cpp `read 12B; str += 12; len -= 12`。
const K_CHUNK: usize = 12;
/// 进入分块循环的最短长度阈值（cpp `while (len >= 32)`，兼容 Lua 5.1）。
const K_MIN_PREFIX_LEN: usize = 32;

/// cpp lstring.cpp `luaS_hash` 对应。原 C 移植签名 `(str: *const c_char, len)` 收窄为字节切片：
/// 长度由切片携带，分块读改用 `from_ne_bytes`（与本机字节序逐位一致），函数体不再含 unsafe。
///
/// Note that this hashing algorithm is replicated in BytecodeBuilder.cpp,
/// BytecodeBuilder::getStringHash —— 哈希值须与字节码侧复刻逐位一致，
/// 分块/倒序遍历次序不可改动；尾段循环收敛至 `ulua_common::lua51_hash_tail` 单点。
pub fn lua_s_hash(str: &[u8]) -> c_uint {
  let mut a: u32 = 0;
  let mut b: u32 = 0;
  let mut h: u32 = str.len() as u32;

  // hash prefix in 12b chunks (using aligned reads) with ARX based hash (LuaJIT v2.1, lookup3)
  // note that we stop at length<32 to maintain compatibility with Lua 5.1
  // cpp 原式 `while (len >= 32) { read 12B; str += 12; len -= 12; }`：
  // 收敛为定长块数 + `as_chunks::<K_CHUNK>`（clippy 定长分块形态），免除逐轮重切；尾段即 remainder，原逻辑不变。
  let blocks = if str.len() >= K_MIN_PREFIX_LEN {
    (str.len() - K_MIN_PREFIX_LEN) / K_CHUNK + 1
  } else {
    0
  };
  let (head, tail) = str.split_at(blocks * K_CHUNK);
  for blk in head.as_chunks::<K_CHUNK>().0 {
    // should compile into fast unaligned reads
    let block = [
      u32::from_ne_bytes(blk[0..4].try_into().unwrap()),
      u32::from_ne_bytes(blk[4..8].try_into().unwrap()),
      u32::from_ne_bytes(blk[8..K_CHUNK].try_into().unwrap()),
    ];

    a = a.wrapping_add(block[0]);
    b = b.wrapping_add(block[1]);
    h = h.wrapping_add(block[2]);

    (a, b, h) = mix(14, 11, 25, a, b, h);
  }

  // original Lua 5.1 hash for compatibility (exact match when len<32)
  // cpp 自 str[len-1] 递减至 str[0]，倒序不可改；单点见 lua51_hash_tail
  lua51_hash_tail(tail, h)
}
