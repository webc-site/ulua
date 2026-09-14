use ulua_ast::{
  records::{ast_stat_block::AstStatBlock, position::Position},
  visit::ast_stat_block_visit,
};

use crate::{
  functions::get_fragment_location::get_fragment_location,
  records::{fragment_region::FragmentRegion, nearest_statement_finder::NearestStatementFinder},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_fragment_region(
  root: *mut AstStatBlock,
  cursor_position: &Position,
) -> FragmentRegion {
  let mut nsf = NearestStatementFinder::new(*cursor_position);
  ast_stat_block_visit(unsafe { &*root }, &mut nsf);

  let parent = if !nsf.parent.is_null() {
    nsf.parent
  } else {
    root
  };

  FragmentRegion {
    fragment_location: unsafe { get_fragment_location(nsf.nearest, cursor_position) },
    nearest_statement: nsf.nearest,
    parent_block: parent,
  }
}
