use ulua_ast::records::parse_options::ParseOptions;

use crate::records::{fixture::Fixture, try_parse_result::TryParseResult};

impl Fixture {
  /// `tryParse` 的抛错版：解析错误即 panic（携带首条消息）。errors 为空时
  /// root 恒非空（Ok 路径返回 parse_chunk 产物）。
  pub fn parse_ex<'a>(&'a mut self, source: &str, options: &ParseOptions) -> TryParseResult<'a> {
    let result = self.try_parse(source, options);
    if !result.errors.is_empty() {
      panic!("ParseErrors: {:?}", result.errors);
    }
    result
  }
}
