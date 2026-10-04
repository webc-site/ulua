use ulua_ast::{records::ast_stat::AstStat, visit::ast_stat_visit_ref};

use crate::records::contains_function_call::ContainsFunctionCall;

/// `&mut AstStat` 即节点在遍历借用期内存活且可独占的类型系统证明，全链路 safe；
/// ContainsFunctionCall 只写自身 result 标志、绝不写 AST。原
/// `from_ref(stat).cast_mut()` 转铸与裸指针门面随签名收窄一并消失。
pub fn contains_function_call(stat: &mut AstStat) -> bool {
  let mut cfc = ContainsFunctionCall::new(false);
  ast_stat_visit_ref(stat, &mut cfc);
  cfc.result
}
