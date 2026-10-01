use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_node::AstNode, ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile,
    ast_type_function::AstTypeFunction, node_handle::OptNode, position::Position,
  },
  rtti::{ast_node_is, ast_node_try_as},
};
pub fn is_valid_break_continue_context(ancestry: &[*mut AstNode], position: Position) -> bool {
  // `ancestry` 槽位为 parse arena 存活节点或 null（records 引用化波次前的裸
  // 指针形态）：逐槽经 `OptNode` 句柄把可空性与借用折进本循环迭代的局部半径，
  // 判型/下转全部走生命周期正确的安全 API，不再锻造 'static。
  for slot in ancestry.iter().rev() {
    let node = OptNode::<AstNode>::from_ptr(*slot);
    let Some(n) = node.get() else {
      continue;
    };

    if ast_node_is::<AstStatFunction>(n)
      || ast_node_is::<AstStatLocalFunction>(n)
      || ast_node_is::<AstExprFunction>(n)
      || ast_node_is::<AstStatTypeFunction>(n)
      || ast_node_is::<AstTypeFunction>(n)
    {
      return false;
    }

    // 循环体 location 覆盖 position 即为合法 break/continue 上下文。
    // while 的 body 已句柄化（`Node` 非空由 parser 构造端兑现）。
    if let Some(stat) = ast_node_try_as::<AstStatWhile>(n)
      && stat.body.base.base.location.contains(position)
    {
      return true;
    }
    if let Some(stat) = ast_node_try_as::<AstStatFor>(n)
      && stat.body.base.base.location.contains(position)
    {
      return true;
    }
    if let Some(stat) = ast_node_try_as::<AstStatForIn>(n)
      && stat.body.base.base.location.contains(position)
    {
      return true;
    }
    if let Some(stat) = ast_node_try_as::<AstStatRepeat>(n)
      && stat.body.base.base.location.contains(position)
    {
      return true;
    }
  }

  false
}
