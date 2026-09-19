use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_break::AstStatBreak,
    ast_stat_continue::AstStatContinue, ast_stat_return::AstStatReturn,
  },
  rtti::ast_node_as,
};

pub fn is_block_terminator(stat: &AstStat) -> bool {
  let stat_ptr = stat as *const AstStat as *mut AstStat;
  unsafe {
    !ast_node_as::<AstStatReturn>(stat_ptr as *mut AstNode).is_null()
      || !ast_node_as::<AstStatBreak>(stat_ptr as *mut AstNode).is_null()
      || !ast_node_as::<AstStatContinue>(stat_ptr as *mut AstNode).is_null()
  }
}
