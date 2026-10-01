use crate::functions::is_whitespace::is_whitespace;

/// `Luau::strip`：去除首尾空白。参考：`Common/src/StringUtils.cpp`。
pub fn strip(s: &str) -> &str {
  s.trim_matches(is_whitespace)
}
