use memchr::memchr;

const SPECIALS: &[u8] = b"^$*+?.([%-";

/// 编译期把特殊字符集合压成 `u128` 位掩码（集合内字节均 < 0x80），
/// 让单次 `.any` 扫描里的成员判定从对 `SPECIALS` 的线性查找降为 O(1) 位测试。
const SPECIAL_MASK: u128 = build_special_mask();

const fn build_special_mask() -> u128 {
  let mut mask: u128 = 0;
  let mut i = 0;
  while i < SPECIALS.len() {
    mask |= 1u128 << SPECIALS[i];
    i += 1;
  }
  mask
}

/// 是否为 pattern 特殊字符：`>= 0x80` 恒非特殊（集合内均为 ASCII），
/// 短路后移位量必 < 128，无越界风险。
#[inline]
const fn is_special(b: u8) -> bool {
  b < 0x80 && (SPECIAL_MASK >> b as u32) & 1 != 0
}

/// cpp `VM/src/lstrlib.cpp:636`：逐 NUL 段扫描，任一特殊字符即返回 0。
///
/// `bytes` 即 pattern 串（cpp 靠其结尾 NUL 分段；此处内嵌 NUL 分段等价，
/// 尾段以切片尽头收口）。
pub(crate) fn nospecials(bytes: &[u8]) -> i32 {
  let mut pos = 0usize;
  loop {
    let rest = &bytes[pos..];
    // cpp 的 strpbrk × strlen 换成单次短路扫描：段 = 至下一个 NUL 前的字节
    let segment = match memchr(0, rest) {
      Some(nul) => &rest[..nul],
      None => rest,
    };
    if segment.iter().any(|&b| is_special(b)) {
      return 0;
    }

    pos += segment.len() + 1; // 跳过段与其 NUL 终止符
    if pos > bytes.len() {
      return 1;
    }
  }
}
