/// `Luau::isIdentifier`：字符集判定（字母、数字、下划线）。
/// 参考：`Common/src/StringUtils.cpp`。
///
/// 字节域等价论证：cpp 逐字节判定，任何非 ASCII 字节（≥ 0x80）必不满足
/// isAlpha/isDigit/`'_'`，返回 false；`chars()` 版对多字节序列解出的非 ASCII
/// 码点同样不命中任何区间，也返回 false——两者对「含非 ASCII 内容」恒 false、
/// 对纯 ASCII 串逐字节同判定（UTF-8 里 ASCII 码位只以单字节出现）。故直接
/// 按字节遍历，免去 UTF-8 解码，与 cpp 单字节循环同形。
pub fn is_identifier(s: &str) -> bool {
  s.as_bytes()
    .iter()
    .all(|&b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
}
