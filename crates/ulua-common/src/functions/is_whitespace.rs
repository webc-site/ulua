/// C++ `isWhitespace`（StringUtils.cpp 的 static 函数）：空格、换行、回车、制表。
pub fn is_whitespace(c: char) -> bool {
  matches!(c, ' ' | '\n' | '\r' | '\t')
}
