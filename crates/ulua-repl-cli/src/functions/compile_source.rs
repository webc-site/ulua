//! 共享编译辅助: 当前 CLI 选项编译, 供 run_code/run_file/load/is_incomplete_chunk 复用

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_compiler::functions::compile::compile;

use crate::functions::copts::copts;

/// 以当前 CLI 编译选项编译 `source`, 返回字节码 blob (错误时为首字节 NUL 的错误 blob)
pub(crate) fn compile_source(source: &str) -> String {
  let options = copts();
  let parse_options = ParseOptions::default();
  compile(source, &options, &parse_options, NoopEncoder)
}
