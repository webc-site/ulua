/// C++ `isWhitespace`（StringUtils.cpp 的 static 函数）：空格、换行、回车、制表。
/// cpp 里即为 internal-linkage static，Rust 侧仅 `strip` 消费，降 `pub(crate)`。
pub(crate) fn is_whitespace(c: char) -> bool {
  matches!(c, ' ' | '\n' | '\r' | '\t')
}
