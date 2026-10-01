/// cpp `isMethodOrFunctionChar` (CLI/src/Repl.cpp): 补全项首字符合法性判断
#[inline]
pub fn is_method_or_function_char(c: u8) -> bool {
  c.is_ascii_alphanumeric() || c == b'.' || c == b':' || c == b'_'
}
