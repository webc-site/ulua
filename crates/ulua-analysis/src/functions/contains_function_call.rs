use core::ptr::from_ref;

use ulua_ast::{records::ast_stat::AstStat, visit::ast_stat_visit};

use crate::records::contains_function_call::ContainsFunctionCall;

pub fn contains_function_call(stat: &AstStat) -> bool {
  let mut cfc = ContainsFunctionCall::new(false);
  let stat_ptr = from_ref(stat).cast_mut();
  // Safety: const->mut 转换仅为匹配 ast_stat_visit 沿袭 C++ 非 const AstVisitor 的签名；
  // ContainsFunctionCall 只写自身 result 标志、绝不写 AST，节点是 arena 中逻辑可变的
  // parser 产物；遍历单线程串行、期间无其他借用者，故无实际写与别名冲突。
  unsafe {
    ast_stat_visit(stat_ptr, &mut cfc);
  }
  cfc.result
}
