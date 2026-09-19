use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal,
    ast_node::AstNode, ast_stat_assign::AstStatAssign,
  },
  rtti::ast_node_as,
};

use crate::records::symbol::Symbol;
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn find_rhs_expr_symbol_ast_stat_assign(
  sym: Symbol,
  source: *mut AstStatAssign,
) -> *mut AstExpr {
  unsafe {
    let source = &*source;
    // zip 在 values 耗尽处停摆，等价于上游对越界 i 的逐个 continue
    for (&var, &value) in source.vars.as_slice().iter().zip(source.values.as_slice()) {
      if !sym.local.is_null() {
        let expr_local = ast_node_as::<AstExprLocal>(var as *mut AstNode);
        if !expr_local.is_null() && (*expr_local).local == sym.local {
          return value;
        }
      } else if !sym.global.is_null() {
        let expr_global = ast_node_as::<AstExprGlobal>(var as *mut AstNode);
        if !expr_global.is_null() && (*expr_global).name == sym.global {
          return value;
        }
      }
    }
    null_mut()
  }
}
