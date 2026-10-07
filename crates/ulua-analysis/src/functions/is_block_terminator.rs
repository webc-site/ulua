use ulua_ast::{
  records::{
    ast_stat::AstStat, ast_stat_break::AstStatBreak, ast_stat_continue::AstStatContinue,
    ast_stat_return::AstStatReturn,
  },
  rtti::ast_node_is,
};

pub(crate) fn is_block_terminator(stat: &AstStat) -> bool {
  // `&AstStat` 实现 AstNodeView，`ast_node_is::<T>` 安全判定 class_index。
  ast_node_is::<AstStatReturn>(stat)
    || ast_node_is::<AstStatBreak>(stat)
    || ast_node_is::<AstStatContinue>(stat)
}
