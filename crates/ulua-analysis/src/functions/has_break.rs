use ulua_ast::{
  records::{
    ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
    ast_stat_if::AstStatIf,
  },
  rtti::{ast_node_is, ast_node_try_as},
};
/// cpp `hasBreak(AstStat*)` 的引用形态：可空性折叠移到调用方的 `Option`
/// （elsebody），树内全部下转走生命周期正确的 `ast_node_try_as`，借用半径
/// 即入参 `node` 的借用（AST 节点存活于 arena，遍历期内只读、单线程）。
pub(crate) fn has_break(node: &AstStat) -> bool {
  if let Some(stat) = ast_node_try_as::<AstStatBlock>(node) {
    // 任一子语句含 break 即可提前返回；`iter` 直接借出存活子语句引用。
    return stat.body.iter().any(has_break);
  }

  if ast_node_is::<AstStatBreak>(node) {
    return true;
  }

  if let Some(stat) = ast_node_try_as::<AstStatIf>(node) {
    // thenbody 句柄恒非空（parser 构造端兑现），elsebody 可空折叠为 is_some_and。
    return has_break(&stat.thenbody.base) || stat.elsebody.get().is_some_and(has_break);
  }

  false
}
