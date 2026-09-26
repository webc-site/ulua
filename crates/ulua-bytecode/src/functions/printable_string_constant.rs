/// 是否可打印：Lua 字符串允许任意字节，此处只要求全部 ≥ 空格（cpp `isPrintableStringConstant`）。
pub(crate) fn printable_string_constant(bytes: &[u8]) -> bool {
  bytes.iter().all(|&b| b >= b' ')
}
