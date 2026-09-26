use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal,
    ast_stat_assign::AstStatAssign, ast_stat_local::AstStatLocal,
  },
  rtti::ast_node_try_as_ptr,
};

use crate::records::symbol::Symbol;

/// 对应 C++ `findRhsExpr(Symbol, AstStatLocal*)`（`cpp/Analysis/src/DumpCFG.cpp:104`）。
///
/// 降 safe 说明：`source` 原为 `*mut AstStatLocal`，函数体只做只读遍历取子节点
/// 指针（vars/values 由 parser 建于同一 arena 并与 source 同寿），故收窄为
/// `&AstStatLocal`，裸指针派生留在调用点（CFG 指令字段）既有 unsafe 处；
/// 返回值的可空 `*mut AstExpr` 维持 C++ nullptr 哨兵语义，调用方判空后才解引用。
pub fn find_rhs_expr_symbol_ast_stat_local(sym: Symbol, source: &AstStatLocal) -> *mut AstExpr {
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

/// 对应 C++ `findRhsExpr(Symbol, AstStatAssign*)`（`cpp/Analysis/src/DumpCFG.cpp:116`）。
/// 降 safe 理由同 `find_rhs_expr_symbol_ast_stat_local`：`source` 只被只读遍历，
/// 收窄为 `&AstStatAssign`；返回可空 `*mut AstExpr` 维持 nullptr 哨兵语义。
pub fn find_rhs_expr_symbol_ast_stat_assign(sym: Symbol, source: &AstStatAssign) -> *mut AstExpr {
  // zip 在 values 耗尽处停摆，等价于上游对越界 i 的逐个 continue
  for (&var, &value) in source.vars.as_slice().iter().zip(source.values.as_slice()) {
    // Safety: var 是 parser arena 存活子节点指针（与 source 同寿、地址不移动），
    // ast_node_try_as_ptr 对 null 返回 None、判型命中才借出只读引用，仅做比较无写入。
    if !sym.local.is_null() {
      if let Some(expr_local) = unsafe { ast_node_try_as_ptr::<AstExprLocal>(var) }
        && expr_local.local.as_ptr() == sym.local
      {
        return value;
      }
    } else if !sym.global.is_null()
      && let Some(expr_global) = unsafe { ast_node_try_as_ptr::<AstExprGlobal>(var) }
      && expr_global.name == sym.global
    {
      return value;
    }
  }
  null_mut()
}
