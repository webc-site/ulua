use ulua_ast::{
  records::{
    ast_stat::AstStat, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction,
  },
  rtti::ast_node_is,
};
pub fn is_function(stat: &AstStat) -> bool {
  // `&AstStat` 实现安全视图 trait `AstNodeView`（经 base 读 class_index），判别与
  // 原「先上转 AstNode 基类视图再判」的指针形态逐字等价（同一 class_index 只读
  // 比较），引用即存活证明，unsafe 与手写 upcast 一并消失。
  ast_node_is::<AstStatFunction>(stat) || ast_node_is::<AstStatLocalFunction>(stat)
}
