use alloc::{boxed::Box, string::String};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_stat::AstStat,
  ast_stat_block::AstStatBlock, fragment_parse_resume_settings::FragmentParseResumeSettings,
  parse_options::ParseOptions, parser::Parser, position::Position,
};

use crate::{
  functions::{
    find_ancestry_at_position_for_autocomplete_ast_query::find_ancestry_at_position_for_autocomplete_ast_stat_block_position,
    find_ancestry_for_fragment_parse::find_ancestry_for_fragment_parse,
    get_document_offsets::get_document_offsets,
  },
  records::fragment_parse_result::FragmentParseResult,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn parse_fragment(
  stale: *mut AstStatBlock,
  most_recent_parse: *mut AstStatBlock,
  names: *mut AstNameTable,
  src: &str,
  cursor_pos: &Position,
  fragment_end_position: Option<Position>,
) -> Option<FragmentParseResult> {
  if most_recent_parse.is_null() {
    return None;
  }

  // Safety: find_ancestry_for_fragment_parse 的契约要求 stale/last_good_parse 为 Frontend 双树轮转
  // 中存活的解析树根，返回值 ancestry/nearest_statement/parent_block 皆指向这些树。stale 为调用方
  // 持有的旧 root；most_recent_parse 已在上一行判非空（对应 C++ lastGoodParse!=nullptr）。两树在
  // 本函数执行期间均存活，且未被别名可变借用。
  let result = unsafe { find_ancestry_for_fragment_parse(stale, *cursor_pos, most_recent_parse) };
  let mut nearest_statement = result.nearest_statement;

  let start_pos = result.fragment_selection_region.begin;
  let end_pos = fragment_end_position.unwrap_or(result.fragment_selection_region.end);
  let (offset_start, parse_length) = get_document_offsets(src, &start_pos, &end_pos);
  let fragment_source = &src[offset_start..offset_start + parse_length];

  let mut fragment_alloc = Box::new(Allocator::new());
  let parse_options = ParseOptions {
    allow_declaration_syntax: false,
    capture_comments: true,
    parse_fragment: Some(FragmentParseResumeSettings::new(
      result.local_map,
      result.local_stack,
      start_pos,
    )),
    ..Default::default()
  };

  // Safety: names 是调用方以 `&mut AstNameTable` 活引用裸化传入的形参（本函数 `# Safety` 契约要求），
  // 非空、对齐且在本次解析内存活；重建的 `&mut` 独占借用无并存别名（fragment_alloc 为本地独占借用）。
  let parse_result = unsafe {
    Parser::parse(
      fragment_source,
      &mut *names,
      &mut fragment_alloc,
      parse_options,
    )
  };

  if parse_result.root.is_null() {
    return None;
  }

  let mut fabricated_ancestry = find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
    // Safety: most_recent_parse 在函数入口已判非空，指向调用方 SourceModule 拥有的存活 AstStatBlock
    // 树，本函数期间存活；被调为安全 fn，重建的 &mut 借用仅存活于本次调用。
    unsafe { &mut *most_recent_parse },
    *cursor_pos,
  );
  let fragment_ancestry = find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
    // Safety: parse_result.root 在上方 `is_null()` 分支已判非空，指向 fragment_alloc 刚解析出的
    // 存活 AstStatBlock（fragment_alloc 存活至函数末尾）；重建 &mut 借用仅存活于本次调用。
    unsafe { &mut *parse_result.root },
    *cursor_pos,
  );

  let mut back = fabricated_ancestry.len();
  for fragment_node in fragment_ancestry.iter().rev() {
    if back == 0 {
      break;
    }

    back -= 1;
    let fabricated_node = fabricated_ancestry[back];
    if !fragment_node.is_null()
      && !fabricated_node.is_null()
      // Safety: 上方 `&&` 短路已保证两指针非空；fragment_node/fabricated_node 分别取自
      // parse_result.root 与 most_recent_parse 两棵在遍历期内存活的 AST，指向的 AstNode 稳定；
      // 仅读取 class_index 做 RTTI 比较。
      && unsafe { (**fragment_node).class_index == (*fabricated_node).class_index }
    {
      fabricated_ancestry[back] = *fragment_node;
    }
  }

  if nearest_statement.is_null() {
    nearest_statement = parse_result.root.cast::<AstStat>();
  }

  let scope_pos = if result.parent_block.is_null() {
    Position { line: 0, column: 0 }
  } else {
    // Safety: 上一分支已判定 parent_block 非空；它指向 find_ancestry 返回、源自存活解析树的
    // AstStatBlock，本函数期间稳定，只读其 location.begin。
    unsafe { (*result.parent_block).base.base.location.begin }
  };

  Some(FragmentParseResult {
    fragment_to_parse: String::from(fragment_source),
    root: parse_result.root,
    ancestry: fabricated_ancestry,
    nearest_statement,
    comment_locations: parse_result.comment_locations,
    alloc: fragment_alloc,
    scope_pos,
  })
}
