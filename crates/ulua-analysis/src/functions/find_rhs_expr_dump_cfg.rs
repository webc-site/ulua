use core::ptr::NonNull;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{ast_expr::AstExpr, ast_stat_assign::AstStatAssign, ast_stat_local::AstStatLocal},
};

use crate::records::symbol::Symbol;

/// 对应 C++ `findRhsExpr(Symbol, AstStatLocal*)`（`cpp/Analysis/src/DumpCFG.cpp:104`）。
///
/// 降 safe 说明：`source` 为 `&AstStatLocal`，函数体只读遍历取子节点引用；
/// 返回 `Option<&'a AstExpr>`。
pub fn find_rhs_expr_symbol_ast_stat_local(sym: Symbol, source: &AstStatLocal) -> Option<&AstExpr> {
  sym.local?;

  // vars/values 按下标一一对应，zip 自动取较短的长度
  for (&var, &value) in source.vars.as_slice().iter().zip(source.values.as_slice()) {
    // cpp `var == sym->local`：纯指针身份比较，两侧各自 `NonNull::new` 归一后比地址。
    if NonNull::new(var) == sym.local {
      return unsafe { value.as_ref() };
    }
  }
  None
}

/// 对应 C++ `findRhsExpr(Symbol, AstStatAssign*)`（`cpp/Analysis/src/DumpCFG.cpp:116`）。
/// `source` 只被只读遍历，返回 `Option<&'a AstExpr>`。
pub fn find_rhs_expr_symbol_ast_stat_assign(
  sym: Symbol,
  source: &AstStatAssign,
) -> Option<&AstExpr> {
  // zip 在 values 耗尽处停摆，等价于上游对越界 i 的逐个 continue
  for (&var, &value) in source.vars.as_slice().iter().zip(source.values.as_slice()) {
    let Some(var_expr) = (unsafe { var.as_ref() }) else {
      continue;
    };
    if let Some(sym_local) = sym.local {
      if let AstExprRef::Local(expr_local) = var_expr.as_expr_ref()
        && NonNull::from(expr_local.local.get()) == sym_local
      {
        return unsafe { value.as_ref() };
      }
    } else if !sym.global.is_null()
      && let AstExprRef::Global(expr_global) = var_expr.as_expr_ref()
      && expr_global.name == sym.global
    {
      return unsafe { value.as_ref() };
    }
  }
  None
}
