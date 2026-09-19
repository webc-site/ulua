use alloc::vec::Vec;

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
  parse_options::ParseOptions, parser::Parser,
};

/// 解析 `source` 并返回根块的独占借用：AST 内存位于 `allocator`，借用期内
/// allocator 不移动/不销毁，调用侧免 unsafe 解引用。返回 `&mut` 与
/// `Fixture::parse` 一致，也让 `trace_requires` 能按 cpp 非 const
/// `AstStatBlock*` 语义拿到独占借用（visitor 写穿节点）。
///
/// 只借 `names` 与 `allocator` 两个字段（而非整个 fixture），调用方即可在持有
/// `block` 的同时独占借用 `fixture.file_resolver`。
pub fn require_tracer_fixture_parse<'a>(
  names: &'a mut Box<AstNameTable>,
  allocator: &'a mut Box<Allocator>,
  source: &str,
) -> &'a mut AstStatBlock {
  names.rebind_allocator(&mut **allocator as *mut Allocator);

  let result = Parser::parse(source, names, allocator, ParseOptions::default());

  assert!(
    result.errors.is_empty(),
    "Parse error: {}",
    result
      .errors
      .iter()
      .map(|error| error.what())
      .collect::<Vec<_>>()
      .join("\n")
  );

  // SAFETY: root 指向 allocator 中存活的根块；`names`/`allocator` 已被独占借用
  // 至 `'a`，借用期内 arena 无其他并发访问。
  unsafe { &mut *result.root }
}
