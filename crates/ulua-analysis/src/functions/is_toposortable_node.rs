use ulua_ast::{
  records::{ast_stat::AstStat, ast_stat_type_alias::AstStatTypeAlias},
  rtti::ast_node_is,
};

use crate::functions::is_function::is_function;
pub fn is_toposortable_node(stat: &AstStat) -> bool {
  // `&AstStat` 实现安全视图 trait `AstNodeView`（经 base 读 class_index），与先经
  // 「上转 AstNode 基类视图再造 `&AstNode`」旧形态读取的 class_index 完全同值，
  // 判别逐字等价；unsafe 与手写 upcast 消失。
  is_function(stat) || ast_node_is::<AstStatTypeAlias>(stat)
}
