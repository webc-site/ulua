use ulua_ast::records::location::Location;

/// 报告错误信息（cpp 各 CLI 的 `reportError` 镜像；bytecode/compile CLI 共用）。
/// 消息均来自 Rust 内部字符串，用 &str 免去 C 字符串 NUL 终止约定，
/// 避免对非 NUL 结尾缓冲读越界。
pub fn report(name: &str, location: &Location, r#type: &str, message: &str) {
  eprintln!(
    "{}({},{}): {}: {}",
    name,
    location.begin.line + 1,
    location.begin.column + 1,
    r#type,
    message
  );
}
