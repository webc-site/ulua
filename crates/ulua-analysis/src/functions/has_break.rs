use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak, ast_stat_if::AstStatIf,
  },
  rtti::{ast_node_as, ast_node_is},
};
pub fn has_break(node: *mut AstStat) -> bool {
  if node.is_null() {
    return false;
  }

  unsafe {
    if !ast_node_as::<AstStatBlock>(node as *mut AstNode).is_null() {
      let stat = ast_node_as::<AstStatBlock>(node as *mut AstNode);
      if stat.is_null() {
        return false;
      }

      for i in 0..(*stat).body.size {
        if has_break(*(*stat).body.data.add(i)) {
          return true;
        }
      }
      return false;
    }

    if ast_node_is::<AstStatBreak>(&*(node as *const AstNode)) {
      return true;
    }

    if !ast_node_as::<AstStatIf>(node as *mut AstNode).is_null() {
      let stat = ast_node_as::<AstStatIf>(node as *mut AstNode);
      if stat.is_null() {
        return false;
      }

      if has_break((*stat).thenbody as *mut AstStat) {
        return true;
      }

      if !(*stat).elsebody.is_null() && has_break((*stat).elsebody) {
        return true;
      }

      return false;
    }

    false
  }
}
