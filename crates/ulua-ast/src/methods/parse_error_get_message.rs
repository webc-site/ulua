use crate::records::parse_error::ParseError;

impl ParseError {
  /// 只读消息文本。cpp 侧只有 `what()`（`Parser.cpp:114` 的 `const char*`），
  /// 这里让两个名字共享同一实现并统一返回 `&str`：`&String` 只是把字段类型
  /// 泄漏给调用方，徒增一层 deref。
  #[inline]
  pub fn get_message(&self) -> &str {
    self.what()
  }
}
