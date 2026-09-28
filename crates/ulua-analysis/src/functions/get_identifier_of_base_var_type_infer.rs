use alloc::string::{String, ToString};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
  },
  rtti::ast_node_try_as_ptr,
};
pub(crate) fn get_identifier_of_base_var(node: *mut AstExpr) -> Option<String> {
  // Safety: `node` 是 AST arena 指针（bump 分配、块地址不移动，借用期内存活）；
  // `ast_node_try_as_ptr` 按 RTTI class-index 分派，null/未命中返回 None 且从不解
  // 引用，命中即按 repr(C) 基址重合借出类型正确的只读引用。递归下行经 `.expr`
  // 子指针，由 parser 保证非空。全程只读、单线程遍历，无别名冲突。
  unsafe {
    if let Some(global) = ast_node_try_as_ptr::<AstExprGlobal>(node) {
      // AstName 由词法器 intern，必为合法 UTF-8；空名（null）按 "" 处理。
      return Some(global.name.as_str_or_empty().to_string());
    }

    if let Some(local) = ast_node_try_as_ptr::<AstExprLocal>(node) {
      return Some(local.local.name.as_str_or_empty().to_string());
    }

    if let Some(index_expr) = ast_node_try_as_ptr::<AstExprIndexExpr>(node) {
      // expr 已句柄化恒非空；递归为既有裸指针 API，经 as_ptr 桥接。
      return get_identifier_of_base_var(index_expr.expr.as_ptr());
    }

    if let Some(index_name) = ast_node_try_as_ptr::<AstExprIndexName>(node) {
      // expr 已句柄化恒非空；递归为既有裸指针 API，经 as_ptr 桥接。
      return get_identifier_of_base_var(index_name.expr.as_ptr());
    }

    None
  }
}
