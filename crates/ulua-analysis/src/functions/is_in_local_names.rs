use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_node::AstNode, ast_stat_block::AstStatBlock,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
    position::Position,
  },
  rtti::{ast_node_as, ast_node_is},
};
pub fn is_in_local_names(ancestry: &[*mut AstNode], position: Position) -> bool {
  let mut iter = ancestry.len();
  while iter > 0 {
    iter -= 1;
    let node = ancestry[iter];
    if node.is_null() {
      continue;
    }

    if unsafe { ast_node_is::<AstStatLocal>(&*node) } {
      let stat_local = unsafe { ast_node_as::<AstStatLocal>(node) };
      let vars = unsafe { &(*stat_local).vars };
      for var in vars {
        let var_location = unsafe { (*(*var)).location };
        if var_location.contains_closed(position) {
          return true;
        }
      }
    } else if unsafe { ast_node_is::<AstExprFunction>(&*node) } {
      let func_expr = unsafe { ast_node_as::<AstExprFunction>(node) };
      let arg_location = unsafe { (*func_expr).arg_location };
      if let Some(arg_loc) = arg_location
        && arg_loc.contains(position)
      {
        return true;
      }
    } else if unsafe { ast_node_is::<AstStatLocalFunction>(&*node) } {
      let local_func = unsafe { ast_node_as::<AstStatLocalFunction>(node) };
      let name_location = unsafe { (*(*local_func).name).location };
      if name_location.contains_closed(position) {
        return true;
      }
    } else if unsafe { ast_node_is::<AstStatBlock>(&*node) } {
      let block = unsafe { ast_node_as::<AstStatBlock>(node) };
      let body = unsafe { &(*block).body };
      if !body.is_empty() {
        return false;
      }
    } else if unsafe { (*node).as_stat_const() }.is_null() {
      // If it's not a stat, continue searching
    } else {
      return false;
    }
  }

  false
}
