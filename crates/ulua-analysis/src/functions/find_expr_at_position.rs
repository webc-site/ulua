use core::ptr::{NonNull, null_mut};

use ulua_ast::records::{ast_expr::AstExpr, position::Position};

use crate::{
  functions::find_node_at_position_ast_query::find_node_at_position_source_module_position,
  records::source_module::SourceModule,
};
pub fn find_expr_at_position(source: &SourceModule, pos: Position) -> *mut AstExpr {
  let node = find_node_at_position_source_module_position(source, pos);
  if !node.is_null() {
    // Safety: 判空后 node 指向 ancestry 中 parser arena 存活节点（块地址不移动）；
    // as_expr 仅按 class_index 判别并做 repr(C) 基址重合的上转，内部只读基类字段，
    // 单线程且经 &mut 瞬时借用，无其他借用者、无别名冲突。
    unsafe { (*node).as_expr() }.map_or(null_mut(), NonNull::as_ptr)
  } else {
    null_mut()
  }
}
