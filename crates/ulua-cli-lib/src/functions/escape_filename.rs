use alloc::string::String;

/// cpp `escapeFilename`: 反斜杠转斜杠, 双引号前加反斜杠转义
pub fn escape_filename(filename: &str) -> String {
  let mut escaped = String::with_capacity(filename.len());

  // cpp 按字节复制, &str 为合法 UTF-8, chars 迭代等价且保留多字节字符
  for ch in filename.chars() {
    match ch {
      '\\' => escaped.push('/'),
      '"' => {
        escaped.push('\\');
        escaped.push('"');
      }
      _ => escaped.push(ch),
    }
  }

  escaped
}
