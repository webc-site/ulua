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
    for i in 0..source.vars.size {
      if i >= source.values.size {
        continue;
      }

      let var = *source.vars.data.add(i);
      if !sym.local.is_null() {
        let expr_local = ast_node_as::<AstExprLocal>(var as *mut AstNode);
        if !expr_local.is_null() && (*expr_local).local == sym.local {
          return *source.values.data.add(i);
        }
      } else if !sym.global.value.is_null() {
        let expr_global = ast_node_as::<AstExprGlobal>(var as *mut AstNode);
        if !expr_global.is_null() && (*expr_global).name.value == sym.global.value {
          return *source.values.data.add(i);
        }
      }
    }
    null_mut()
  }
}
