use crate::{
  enums::type_lexer::Type,
  functions::optional_node::node_opt,
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, parse_error::ParseError,
    parse_node_result::ParseNodeResult, parse_options::ParseOptions, parser::Parser,
  },
};

impl Parser {
  /// cpp 模板 `Parser::runParse(buffer, bufferSize, ..., F f)`
  /// （`Ast/src/Parser.cpp:248`）：单节点形态的解析入口。
  ///
  /// `buffer` 为原始源码字节，长度即 `buffer.len()`（无独立的 size 参数）。
  pub fn run_parse<Node, F>(
    buffer: &[u8],
    names: &mut AstNameTable,
    allocator: &mut Allocator,
    options: ParseOptions,
    f: F,
  ) -> ParseNodeResult<Node>
  where
    F: FnOnce(&mut Parser) -> *mut Node,
  {
    Self::guarded_parse(buffer, names, allocator, options, f, |p, root| {
      // cpp: `Lexeme eof = p.lexer.next(); if (eof.type != Lexeme::Eof) { expr = nullptr; parseErrors.push_back(...); }`
      let eof = p.lexer.next_lexeme();

      if eof.r#type != Type::EOF {
        p.parse_errors
          .push(ParseError::new(eof.location, "Expected end of file".into()));
        // 尾部仍有内容：cpp 把结果指针重置为 nullptr，这里用 `None` 表达同一「根作废」态，
        // 裸指针只在 guarded_parse 出口经 opt_node 折回一次。
        return None;
      }

      node_opt(root)
    })
  }
}
