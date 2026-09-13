use ulua_ast::{
  records::{
    ast_stat::AstStat, ast_stat_block::AstStatBlock, location::Location, position::Position,
  },
  visit::ast_stat_block_visit,
};

use crate::{
  functions::{block_diff_start::block_diff_start, get_fragment_location::get_fragment_location},
  records::{
    fragment_region::FragmentRegion, nearest_likely_block_finder::NearestLikelyBlockFinder,
    nearest_statement_finder::NearestStatementFinder,
  },
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_fragment_region_with_block_diff(
  stale: *mut AstStatBlock,
  fresh: *mut AstStatBlock,
  cursor_pos: &Position,
) -> FragmentRegion {
  let mut nsf = NearestStatementFinder::new(*cursor_pos);
  ast_stat_block_visit(unsafe { &*fresh }, &mut nsf);

  let parent = if !nsf.parent.is_null() {
    nsf.parent
  } else {
    fresh
  };

  let nearest = if !nsf.nearest.is_null() {
    nsf.nearest
  } else {
    fresh as *mut AstStat
  };

  let mut lsf = NearestLikelyBlockFinder::new(parent);
  // C++ `stale->visit(&lsf)` — traverse the entire stale AST so the visitor
  // sees every nested block (e.g. the inner `do` block), not just the root.
  ast_stat_block_visit(unsafe { &*stale }, &mut lsf);

  if let Some(same_block) = lsf.found
    && let Some(fd) = unsafe { block_diff_start(same_block, parent, nearest) }
  {
    return FragmentRegion {
      fragment_location: Location::new(fd, *cursor_pos),
      nearest_statement: nearest,
      parent_block: parent,
    };
  }

  FragmentRegion {
    fragment_location: unsafe { get_fragment_location(nsf.nearest, cursor_pos) },
    nearest_statement: nearest,
    parent_block: parent,
  }
}
