use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_node::AstNode, ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile,
    ast_type_function::AstTypeFunction, position::Position,
  },
  rtti::ast_node_is,
};
pub fn is_valid_break_continue_context(ancestry: &[*mut AstNode], position: Position) -> bool {
  let mut iter = ancestry.len();
  while iter > 0 {
    iter -= 1;
    let node = ancestry[iter];

    if unsafe { ast_node_is::<AstStatFunction>(&*node) }
      || unsafe { ast_node_is::<AstStatLocalFunction>(&*node) }
      || unsafe { ast_node_is::<AstExprFunction>(&*node) }
      || unsafe { ast_node_is::<AstStatTypeFunction>(&*node) }
      || unsafe { ast_node_is::<AstTypeFunction>(&*node) }
    {
      return false;
    }

    if let Some(stat_while) =
      unsafe { ast_node_is::<AstStatWhile>(&*node).then_some(node as *mut AstStatWhile) }
    {
      let body_location = unsafe { (*(*stat_while).body).base.base.location };
      if body_location.contains(position) {
        return true;
      }
    }

    if let Some(stat_for) =
      unsafe { ast_node_is::<AstStatFor>(&*node).then_some(node as *mut AstStatFor) }
    {
      let body_location = unsafe { (*(*stat_for).body).base.base.location };
      if body_location.contains(position) {
        return true;
      }
    }

    if let Some(stat_for_in) =
      unsafe { ast_node_is::<AstStatForIn>(&*node).then_some(node as *mut AstStatForIn) }
    {
      let body_location = unsafe { (*(*stat_for_in).body).base.base.location };
      if body_location.contains(position) {
        return true;
      }
    }

    if let Some(stat_repeat) =
      unsafe { ast_node_is::<AstStatRepeat>(&*node).then_some(node as *mut AstStatRepeat) }
    {
      let body_location = unsafe { (*(*stat_repeat).body).base.base.location };
      if body_location.contains(position) {
        return true;
      }
    }
  }

  false
}
