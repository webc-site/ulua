use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal},
  rtti::ast_node_try_as,
};

use crate::records::symbol::Symbol;
pub fn extract_l_value_symbol(target: &AstExpr) -> Option<Symbol> {
  // safe 引用门面：`ast_node_try_as` 与被替换的
  // 「裸指针下转后判 is_null」旧形态
  // 共用同一 class_index 甄别核心（repr(C) 基址重合），命中进 Option、未命中
  // None，判空分支与 null 分支一一对应；全程只读，unsafe 消失。
  if let Some(local) = ast_node_try_as::<AstExprLocal>(&target.base) {
    // local 槽已句柄化恒非空；Symbol::from_local 为既有裸指针 API，经 as_ptr 桥接。
    return Some(Symbol::from_local(local.local.as_ptr()));
  }

  if let Some(global) = ast_node_try_as::<AstExprGlobal>(&target.base) {
    return Some(Symbol::from_global(global.name));
  }

  None
}
