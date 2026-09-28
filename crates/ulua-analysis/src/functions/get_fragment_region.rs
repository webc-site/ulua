use ulua_ast::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{ast_stat_block::AstStatBlock, position::Position},
};

use crate::{
  functions::get_fragment_location::get_fragment_location,
  records::{fragment_region::FragmentRegion, nearest_statement_finder::NearestStatementFinder},
};

/// # Safety
/// - `root` 须为非空、对齐且指向 arena 存活 `AstStatBlock` 的句柄，存活期覆盖本次遍历；
/// - visit 期间该块无其他并存借用（对应 C++ getFragmentRegion 调用契约）。
pub unsafe fn get_fragment_region(
  root: *mut AstStatBlock,
  cursor_position: &Position,
) -> FragmentRegion {
  let mut nsf = NearestStatementFinder::new(*cursor_position);
  // Safety: root 由契约保证为 arena 存活的非空对齐 AstStatBlock；本函数内该块无其他
  // 并存借用，重建的 &mut 仅交给 visit 在单线程遍历期间独占使用。
  ast_stat_block_visit(unsafe { &mut *root }, &mut nsf);

  let parent = if !nsf.parent.is_null() {
    nsf.parent
  } else {
    root
  };

  FragmentRegion {
    fragment_location: unsafe {
      // Safety: 调用 unsafe fn get_fragment_location；nsf.nearest 由 visit 写入，为
      // arena 存活 AST 节点或 null，get_fragment_location 内部对 null 先行判空。
      get_fragment_location(nsf.nearest, cursor_position)
    },
    nearest_statement: nsf.nearest,
    parent_block: parent,
  }
}
