/// `Luau::isIdentifier`：字符集判定（字母、数字、下划线）。
/// 参考：`Common/src/StringUtils.cpp`。
pub fn is_identifier(s: &str) -> bool {
  s.chars()
    .all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
}
