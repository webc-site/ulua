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
  records::{
    arena_handle::{Handle, alias, alias_ref},
    fragment_parse_result::FragmentParseResult,
  },
};

/// 前置契约（本函数体经 safe 门面完成指针借用，无 unsafe 操作；以下为文档约定）
/// 调用方须保证满足 C++ 原实现的调用契约：`names` 指向宿主保活的
/// `AstNameTable`（cpp `module->names.get()` 同形，下游按 `AstNameTable&`
/// 使用），两棵解析树根句柄指向各自 arena 内存活 AST。
pub fn parse_fragment(
  stale: Option<Handle<AstStatBlock>>,
  most_recent_parse: Option<Handle<AstStatBlock>>,
  names: *mut AstNameTable,
  src: &str,
  cursor_pos: &Position,
  fragment_end_position: Option<Position>,
) -> Option<FragmentParseResult> {
  // cpp `if (!lastGoodParse) return nullopt`：`None` ≡ null 早退，不解引用。
  let most_recent_parse = most_recent_parse?;

  // §2：stale/last_good_parse 句柄化直传——双树轮转中存活的解析树根，返回值
  // ancestry/nearest_statement/parent_block 皆指向这些树（Handle 模块契约）。
  let result = find_ancestry_for_fragment_parse(stale, *cursor_pos, Some(most_recent_parse));
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

  let parse_result = Parser::parse(
    fragment_source,
    alias(names),
    &mut fragment_alloc,
    parse_options,
  );

  if parse_result.root.is_null() {
    return None;
  }
  // cpp `parseFragment` 成功路径在上方判空后直取非空根（FragmentAutocomplete.cpp），
  // `Handle::from_ptr` 收口为会话 arena 非空句柄；目标随本结果自带的 `alloc` 存续。
  let root = Handle::from_ptr(parse_result.root);

  let mut fabricated_ancestry = find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
    most_recent_parse.get_mut(),
    *cursor_pos,
  );
  let fragment_ancestry =
    find_ancestry_at_position_for_autocomplete_ast_stat_block_position(root.get_mut(), *cursor_pos);

  let mut back = fabricated_ancestry.len();
  for fragment_node in fragment_ancestry.iter().rev() {
    if back == 0 {
      break;
    }

    back -= 1;
    let fabricated_node = fabricated_ancestry[back];
    if !fragment_node.is_null()
      && !fabricated_node.is_null()
      && alias_ref(*fragment_node).class_index == alias_ref(fabricated_node).class_index
    {
      fabricated_ancestry[back] = *fragment_node;
    }
  }

  // cpp `nearestStatement ?: (AstStat*)parseResult.root`：回退为根块的
  // `AstStat` 视图（repr(C) 首字段基址重合，原 `.cast::<AstStat>()` 同形）。
  if nearest_statement.is_none() {
    nearest_statement = Some(root.cast::<AstStat>());
  }

  let scope_pos = result
    .parent_block
    .map_or(Position { line: 0, column: 0 }, |h| {
      h.get().base.base.location.begin
    });

  Some(FragmentParseResult {
    fragment_to_parse: String::from(fragment_source),
    root,
    ancestry: fabricated_ancestry,
    nearest_statement,
    comment_locations: parse_result.comment_locations,
    alloc: fragment_alloc,
    scope_pos,
  })
}
