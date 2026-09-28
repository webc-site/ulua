use core::ptr::null_mut;

use ulua_ast::{
  records::{ast_stat::AstStat, ast_stat_if::AstStatIf, position::Position},
  rtti::{ast_node_is, ast_node_try_as_ptr},
};

pub(crate) fn get_nearest_if_to_cursor(
  stmt: *mut AstStat,
  cursor_pos: &Position,
) -> *mut AstStatIf {
  let mut current = stmt;

  // Safety: `stmt` 为 AST arena 指针；`ast_node_try_as_ptr::<AstStatIf>` 按 class-index 下转，
  // 未命中返回 None、命中即类型正确；AST 节点存活于 arena，`cursor_pos` 为存活引用。
  while let Some(current_if) = unsafe { ast_node_try_as_ptr::<AstStatIf>(current) } {
    let elsebody = current_if.elsebody;
    if let Some(else_stat) = elsebody.get()
      && else_stat.base.location.contains_closed(*cursor_pos)
      && ast_node_is::<AstStatIf>(&else_stat.base)
    {
      current = elsebody.as_ptr();
    } else {
      return (current_if as *const AstStatIf).cast_mut();
    }
  }

  null_mut()
}
