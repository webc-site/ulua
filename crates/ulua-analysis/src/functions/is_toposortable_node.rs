use ulua_ast::{
  records::{ast_node::AstNode, ast_stat::AstStat, ast_stat_type_alias::AstStatTypeAlias},
  rtti::ast_node_is,
};

use crate::functions::is_function::is_function;
pub fn is_toposortable_node(stat: &AstStat) -> bool {
  is_function(stat)
    || ast_node_is::<AstStatTypeAlias>(unsafe {
      &*(stat as *const AstStat as *mut AstStat as *mut AstNode)
    })
}
