use ulua_ast::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{ast_stat_block::AstStatBlock, position::Position},
};

use crate::{
  functions::get_fragment_location::get_fragment_location,
  records::{
    arena_handle::Handle, fragment_region::FragmentRegion,
    nearest_statement_finder::NearestStatementFinder,
  },
};

/// §2：入口 `root` 已句柄化——cpp `getFragmentRegion` 对非空 `AstStatBlock*`
/// 的直取遍历（未命中时以根块作 `parentBlock` 兜底），非空性由 [`Handle`]
/// 类型编码、arena 保活契约覆盖本次遍历，`unsafe` 随裸形参一并消亡。
pub fn get_fragment_region(
  root: Handle<AstStatBlock>,
  cursor_position: &Position,
) -> FragmentRegion {
  let mut nsf = NearestStatementFinder::new(*cursor_position);
  ast_stat_block_visit(root.get_mut(), &mut nsf);

  // cpp `parentBlock = nsf.parent ? nsf.parent : root`：未命中回退根块。
  let parent = nsf.parent.or(Some(root));

  FragmentRegion {
    fragment_location: get_fragment_location(nsf.nearest.map(|h| h.get()), cursor_position),
    nearest_statement: nsf.nearest,
    parent_block: parent,
  }
}
