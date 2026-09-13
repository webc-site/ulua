use core::ptr::null_mut;

use ulua_ast::records::{ast_expr::AstExpr, ast_stat_local::AstStatLocal};

use crate::records::symbol::Symbol;
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn find_rhs_expr_symbol_ast_stat_local(
  sym: Symbol,
  source: *mut AstStatLocal,
) -> *mut AstExpr {
  unsafe {
    let source = &*source;
    if sym.local.is_null() {
      return null_mut();
    }

    // vars/values 按下标一一对应，zip 自动取较短的长度
    for (&var, &value) in source.vars.as_slice().iter().zip(source.values.as_slice()) {
      if var == sym.local {
        return value;
      }
    }
    null_mut()
  }
}
