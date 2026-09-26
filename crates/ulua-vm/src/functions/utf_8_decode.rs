use crate::macros::maxunicode::MAXUNICODE;

const MAX_UNICODE: u32 = MAXUNICODE as u32;
const LIMITS: [u32; 4] = [0xFF, 0x7F, 0x7FF, 0xFFFF];

/// cpp `utf8_decode`（`VM/src/lutf8lib.cpp:26-53`）的直译。
///
/// 从 `bytes[0]` 解码一个 UTF-8 序列；成功返回 `(下一字节位置, 码点)`，
/// 序列非法时返回 `(0, None)`（对应 cpp 返回 NULL；cpp 的 `int* val`
/// 出参折叠为元组第二项，cpp 失败时不写 val、调用方也不读取，语义一致）。
///
/// cpp 逐字节推进（lutf8lib.cpp:40 `int cc = s[++count]`），只在头字节
/// 标志位声称的范围内读取：Lua 字符串恒有 NUL 终止（非续字节），读到非法
/// 字节即止。Rust 版以有界切片替代裸指针读：约定 `bytes` 覆盖到含串尾
/// NUL 终止符为止（即长度 = 剩余字节数 + 1），续字节循环必在 NUL 处或
/// 之前停止；`get` 越界分支与「读到串尾外的 NUL」语义等同（NUL 恒被判为
/// 非法续字节），故两端行为一致。
pub(crate) fn utf_8_decode(bytes: &[u8]) -> (usize, Option<u32>) {
  // cpp 的 `if (o == nullptr) return nullptr`：空窗口即失败哨兵
  let Some(&first) = bytes.first() else {
    return (0, None);
  };
  let mut c = first;
  if c < 0x80 {
    // ASCII：res = c，cpp 返回 o + 1
    return (1, Some(u32::from(c)));
  }
  let mut count: usize = 0; // 续字节计数
  let mut res: u32 = 0;
  while c & 0x40 != 0 {
    count += 1;
    let Some(&cc) = bytes.get(count) else {
      return (0, None); // 越界等价读到终止 NUL：非续字节，判非法
    };
    if cc & 0xC0 != 0x80 {
      return (0, None); // 非法续字节
    }
    res = (res << 6) | u32::from(cc & 0x3F);
    c <<= 1; // 检查头部字节的下一位
  }
  res |= u32::from(c & 0x7F) << (count * 5);
  if count > 3 || res > MAX_UNICODE || res <= LIMITS[count] {
    return (0, None); // 超长编码/超范围/多余续字节
  }
  if res.wrapping_sub(0xD800) < 0x800 {
    return (0, None); // 代理区码点
  }
  // cpp lutf8lib.cpp:50-52：跳过已读续字节再 +1 含首字节
  (count + 1, Some(res))
}

/// UTF-8 续字节判定（与 `macros::iscont` 同一模式 10xxxxxx），字节版安全入参。
#[inline]
pub(crate) fn is_cont_byte(c: u8) -> bool {
  c & 0xC0 == 0x80
}

// 留证：被测 `utf_8_decode` 对应 cpp lutf8lib.cpp:26 的 `static` 文件内部函数，
// 无 C ABI 导出面；公开 API（utf8len/utf8codes 等）会吞掉「下一字节位置」且
// 无法逐通道钉死超长编码/代理区/上限外码点的拒绝分支，迁 tests/ 需先破封装。
#[cfg(test)]
mod tests {
  use super::*;

  /// 以本地 NUL 结尾缓冲验证 `utf_8_decode`，对应 cpp lutf8lib 的解码语义。
  /// 入参切片覆盖到含终止 NUL（Lua 字符串布局），与调用方构造方式一致。
  fn decode(buf: &[u8]) -> Option<(u32, usize)> {
    let mut v = buf.to_vec();
    v.push(0); // Lua 字符串恒有 NUL 终止
    let (step, code) = utf_8_decode(&v);
    Some((code?, step))
  }

  #[test]
  fn ascii_and_null() {
    assert_eq!(decode(b"hello"), Some((b'h' as u32, 1)));
    assert_eq!(decode(b""), Some((0, 1))); // NUL 是合法 ASCII
  }

  #[test]
  fn multibyte() {
    // 2-byte: U+00A2 '¢' -> [0xC2, 0xA2]
    assert_eq!(decode(&[0xC2, 0xA2]), Some((0xA2, 2)));
    // 3-byte: U+20AC '€' -> [0xE2, 0x82, 0xAC]
    assert_eq!(decode(&[0xE2, 0x82, 0xAC]), Some((0x20AC, 3)));
    // 4-byte: U+1F600 -> [0xF0, 0x9F, 0x98, 0x80]
    assert_eq!(decode(&[0xF0, 0x9F, 0x98, 0x80]), Some((0x1F600, 4)));
  }

  #[test]
  fn invalid() {
    // 截断（以 NUL 终止）
    assert_eq!(decode(&[0xC2]), None);
    assert_eq!(decode(&[0xE2, 0x82]), None);
    // 非法续字节
    assert_eq!(decode(&[0xC2, 0x00]), None);
    // 超长编码：U+0020 编为 2 字节 [0xC0, 0xA0]（res 0 <= LIMITS[1]）
    assert_eq!(decode(&[0xC0, 0xA0]), None);
    // 代理区：U+D800 -> [0xED, 0xA0, 0x80]
    assert_eq!(decode(&[0xED, 0xA0, 0x80]), None);
    // 超出 Unicode 上限：> 0x10FFFF
    assert_eq!(decode(&[0xF4, 0x90, 0x80, 0x80]), None);
    // 0xFF 全 1 头部字节：首个非续字节（NUL）即判非法
    assert_eq!(decode(&[0xFF]), None);
  }
}
