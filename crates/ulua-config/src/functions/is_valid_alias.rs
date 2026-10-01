/// 对应 C++ `isValidAlias`：非空、非路径，首字符可 '@'，其余仅限字母数字与 `-_.`。
pub(crate) fn is_valid_alias(alias: &str) -> bool {
  if alias.is_empty() || alias == "." || alias == ".." || alias.contains(['\\', '/']) {
    return false;
  }

  let valid = |b: u8| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.';
  let Some((&first, rest)) = alias.as_bytes().split_first() else {
    return false;
  };
  (first == b'@' || valid(first)) && rest.iter().copied().all(valid)
}
