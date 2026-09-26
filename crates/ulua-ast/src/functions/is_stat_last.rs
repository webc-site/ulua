use crate::{
  records::{
    ast_stat::AstStat, ast_stat_break::AstStatBreak, ast_stat_continue::AstStatContinue,
    ast_stat_return::AstStatReturn,
  },
  rtti::ast_node_is,
};

pub fn is_stat_last(stat: &AstStat) -> bool {
  let node = &stat.base;
  ast_node_is::<AstStatBreak>(node)
    || ast_node_is::<AstStatContinue>(node)
    || ast_node_is::<AstStatReturn>(node)
}
