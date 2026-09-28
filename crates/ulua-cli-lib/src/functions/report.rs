use ulua_ast::records::location::Location;

/// 报告错误信息（cpp 各 CLI 的 `reportError` 镜像；bytecode/compile CLI 共用）。
/// 消息均来自 Rust 内部字符串，用 &str 免去 C 字符串 NUL 终止约定，
/// 避免对非 NUL 结尾缓冲读越界。
pub fn report(name: &str, location: &Location, r#type: &str, message: &str) {
  eprint!("{}", report_string(name, location, r#type, message));
}

/// [`report`] 的无副作用版本：返回同一行消息（含结尾换行）。
/// 仅本 crate 的 compile/parse 错误门面（并行编译按文件缓冲输出）消费，
/// 不对外导出。
pub(crate) fn report_string(
  name: &str,
  location: &Location,
  r#type: &str,
  message: &str,
) -> String {
  format!(
    "{}({},{}): {}: {}\n",
    name,
    location.begin.line + 1,
    location.begin.column + 1,
    r#type,
    message
  )
}
