use alloc::{boxed::Box, string::String};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_stat::AstStat,
  ast_stat_block::AstStatBlock, fragment_parse_resume_settings::FragmentParseResumeSettings,
  parse_options::ParseOptions, parser::Parser, position::Position,
};

use crate::{
  functions::{
    find_ancestry_at_position_for_autocomplete_ast_query_alt_b::find_ancestry_at_position_for_autocomplete_ast_stat_block_position,
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

  let parse_result = unsafe {
    Parser::parse(
      fragment_source,
      parse_length,
      &mut *names,
      &mut fragment_alloc,
      parse_options,
    )
  };

  if parse_result.root.is_null() {
    return None;
  }

  let mut fabricated_ancestry = find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
    unsafe { &mut *most_recent_parse },
    *cursor_pos,
  );
  let fragment_ancestry = find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
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
      && unsafe { (**fragment_node).class_index == (*fabricated_node).class_index }
    {
      fabricated_ancestry[back] = *fragment_node;
    }
  }

  if nearest_statement.is_null() {
    nearest_statement = parse_result.root as *mut AstStat;
  }

  let scope_pos = if result.parent_block.is_null() {
    Position { line: 0, column: 0 }
  } else {
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
