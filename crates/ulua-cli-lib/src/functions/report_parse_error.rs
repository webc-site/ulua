use ulua_ast::records::parse_error::ParseError;

use crate::functions::report::report_string;

/// cpp `reportError(name, ParseError)` 的无副作用形态（无消费者直接打印的
/// 变体已随死代码清理删除）：bytecode/compile 两个 CLI 经并行编译按文件缓冲输出。
pub(crate) fn report_parse_error_string(name: &str, error: &ParseError) -> String {
  report_string(name, error.get_location(), "SyntaxError", error.what())
}
