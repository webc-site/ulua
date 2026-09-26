/// Lua 5.1 字符串哈希尾循环：`h ^= (h << 5) + (h >> 2) + c`，自末字节向前。
///
/// ulua-vm 的 `luaS_hash`（cpp lstring.cpp，长串走 12 字节分块后进入本尾段）
/// 与 ulua-bytecode 的 `getStringHash`（cpp BytecodeBuilder.cpp:1293 刻意省略
/// 长串处理，全串直接进本尾段）的公共部分，收敛为单一实现保证逐位一致。
/// 迭代次序 cpp 端为 `str[len-1]` 递减至 `str[0]`，倒序不可改。
#[inline]
pub fn lua51_hash_tail(bytes: &[u8], mut h: u32) -> u32 {
  for &b in bytes.iter().rev() {
    h ^= (h << 5).wrapping_add(h >> 2).wrapping_add(b as u32);
  }
  h
}
