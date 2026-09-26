use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_node::AstNode, ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile,
    ast_type_function::AstTypeFunction, position::Position,
  },
  rtti::{ast_node_is_ptr, ast_node_try_as_ptr},
};
pub fn is_valid_break_continue_context(ancestry: &[*mut AstNode], position: Position) -> bool {
  let mut iter = ancestry.len();
  while iter > 0 {
    iter -= 1;
    let node = ancestry[iter];

    // Safety: ancestry 槽位为 parse arena 存活节点或 null，边界门面只读 class_index。
    if unsafe {
      ast_node_is_ptr::<AstStatFunction>(node)
        || ast_node_is_ptr::<AstStatLocalFunction>(node)
        || ast_node_is_ptr::<AstExprFunction>(node)
        || ast_node_is_ptr::<AstStatTypeFunction>(node)
        || ast_node_is_ptr::<AstTypeFunction>(node)
    } {
      return false;
    }

    // 循环体 location 覆盖 position 即为合法 break/continue 上下文。
    // Safety: 下转命中的借用指向 arena 存活节点（SourceModule 拥有整棵 AST，遍历
    // 期内只读）；while 的 body 已句柄化（`Node` 非空由 parser 构造端兑现）。
    if let Some(stat) = unsafe { ast_node_try_as_ptr::<AstStatWhile>(node) }
      && stat.body.base.base.location.contains(position)
    {
      return true;
    }
    if let Some(stat) = unsafe { ast_node_try_as_ptr::<AstStatFor>(node) }
      // body 已句柄化为 Node：`.get()` 即安全只读视图，死 unsafe 消解。
      && stat.body.get().base.base.location.contains(position)
    {
      return true;
    }
    if let Some(stat) = unsafe { ast_node_try_as_ptr::<AstStatForIn>(node) }
      && stat.body.get().base.base.location.contains(position)
    {
      return true;
    }
    if let Some(stat) = unsafe { ast_node_try_as_ptr::<AstStatRepeat>(node) }
      // body 已句柄化为 Node：`.get()` 即安全只读视图，死 unsafe 消失。
      && stat.body.get().base.base.location.contains(position)
    {
      return true;
    }
  }

  false
}
