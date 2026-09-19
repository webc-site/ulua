use crate::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
  parse_result::ParseResult, parser::Parser,
};

impl Parser {
  /// cpp `Parser::parse(buffer, bufferSize, ...)`（`Ast/src/Parser.cpp:226`）。
  ///
  /// `buffer` 是原始源码字节：Luau 的词法器按字节工作，源文件不要求是合法
  /// UTF-8（`"\xFF"` 这类字面量允许出现在任意位置）。长度直接取自缓冲本身，
  /// 不再有可与其漂移的独立 `buffer_size` 参数。
  pub fn parse<B>(
    buffer: &B,
    names: &mut AstNameTable,
    allocator: &mut Allocator,
    options: ParseOptions,
  ) -> ParseResult
  where
    B: AsRef<[u8]> + ?Sized,
  {
    let outcome = Self::guarded_parse(
      buffer.as_ref(),
      names,
      allocator,
      options,
      |p| p.parse_chunk(),
      // parseChunk 已吞到 EOF，cpp 的 `parse` 无 `runParse` 的 EOF 校验步骤。
      |_, root| root,
    );

    ParseResult {
      root: outcome.root,
      lines: outcome.lines,
      hotcomments: outcome.hotcomments,
      errors: outcome.errors,
      comment_locations: outcome.comment_locations,
      cst_node_map: outcome.cst_node_map,
    }
  }
}
