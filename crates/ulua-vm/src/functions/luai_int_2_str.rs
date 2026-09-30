/// lua_Integer → 十进制字符串热路径。
///
/// itoa 的十进制输出与 `core::fmt` 完全一致，直接回填缓冲区并保持 NUL 结尾。
/// 返回写出长度（不含 NUL）。
///
/// 前置条件：`buf.len() >= LUAI_MAXINT2STR`（i64 十进制最长 20 字符 + NUL），
/// 不满足时按越界 panic（原指针版为 UB），调用点均以定长数组满足。
pub(crate) fn luai_int2str(buf: &mut [u8], l: i64) -> usize {
  use itoa::Buffer;

  let mut b = Buffer::new();
  let s = b.format(l).as_bytes();
  buf[..s.len()].copy_from_slice(s);
  buf[s.len()] = 0; // NUL 结尾，与 C `LUA_INT2STR` 语义一致
  s.len()
}
