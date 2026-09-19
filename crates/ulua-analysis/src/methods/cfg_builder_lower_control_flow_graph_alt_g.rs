//! Source: `Analysis/src/ControlFlowGraph.cpp:253-256` (hand-ported)
//! C++ `void CFGBuilder::lower(AstStatExpr* stat)`.
use ulua_ast::records::ast_stat_expr::AstStatExpr;

use crate::records::cfg_builder::CfgBuilder;

impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn lower_ast_stat_expr(&mut self, stat: *mut AstStatExpr) {
    unsafe {
      // lowerExpr(stat->expr);
      let expr = (*stat).expr;
      self.lower_expr_ast_expr(expr);
    }
  }
}
