use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_node::AstNode, ast_stat_block::AstStatBlock,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
    position::Position,
  },
  rtti::ast_node_try_as,
};
pub fn is_in_local_names(ancestry: &[*mut AstNode], position: Position) -> bool {
  for &node in ancestry.iter().rev() {
    if node.is_null() {
      continue;
    }
    // SAFETY: 非空已判，节点由解析器 arena 持有。
    let node_ref = unsafe { &*node };

    if let Some(stat_local) = ast_node_try_as::<AstStatLocal>(node_ref) {
      for var in stat_local.vars.iter() {
        // SAFETY: vars 元素为解析器分配的存活 AstLocal。
        let var_location = unsafe { (**var).location };
        if var_location.contains_closed(position) {
          return true;
        }
      }
    } else if let Some(func_expr) = ast_node_try_as::<AstExprFunction>(node_ref) {
      if let Some(arg_loc) = func_expr.arg_location
        && arg_loc.contains(position)
      {
        return true;
      }
    } else if let Some(local_func) = ast_node_try_as::<AstStatLocalFunction>(node_ref) {
      // name 已句柄化为 Node<AstLocal>（parser 保证非空由类型层承载），get 即安全只读视图。
      let name_location = local_func.name.get().location;
      if name_location.contains_closed(position) {
        return true;
      }
    } else if let Some(block) = ast_node_try_as::<AstStatBlock>(node_ref) {
      if !block.body.is_empty() {
        return false;
      }
    } else if node_ref.as_stat_const().is_none() {
      // If it's not a stat, continue searching
    } else {
      return false;
    }
  }
  false
}
