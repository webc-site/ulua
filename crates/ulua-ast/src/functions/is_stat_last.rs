use crate::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_break::AstStatBreak,
    ast_stat_continue::AstStatContinue, ast_stat_return::AstStatReturn,
  },
  rtti::ast_node_is,
};

pub fn is_stat_last(stat: *mut AstStat) -> bool {
  ast_node_is::<AstStatBreak>(unsafe { &*(stat as *mut AstNode) })
    || ast_node_is::<AstStatContinue>(unsafe { &*(stat as *mut AstNode) })
    || ast_node_is::<AstStatReturn>(unsafe { &*(stat as *mut AstNode) })
}
