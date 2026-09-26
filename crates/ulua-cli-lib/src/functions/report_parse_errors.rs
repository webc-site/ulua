use ulua_ast::records::parse_error::ParseError;

use crate::functions::report_parse_error::report_parse_error_string;

/// 解析错误非空即逐条上报并返回 `true`（调用方据此止步返回失败），
/// 空时返回 `false` 继续管线；bytecode/compile 两 CLI 同构循环收口。
pub fn report_parse_errors(name: &str, errors: &[ParseError]) -> bool {
  if errors.is_empty() {
    return false;
  }
  eprint!("{}", collect_parse_errors(name, errors));
  true
}

/// [`report_parse_errors`] 的无副作用版本：把逐条消息合并为单个字符串
/// （无错误时为空串），供并行编译按文件缓冲、主线程按原顺序统一输出。
pub fn collect_parse_errors(name: &str, errors: &[ParseError]) -> String {
  let mut message = String::new();
  for error in errors {
    message.push_str(&report_parse_error_string(name, error));
  }
  message
}
