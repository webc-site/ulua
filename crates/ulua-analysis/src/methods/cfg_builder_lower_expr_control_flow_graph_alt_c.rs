//! Source: `Analysis/src/ControlFlowGraph.cpp:470-477` (hand-ported)
//! C++ `void CFGBuilder::lowerExpr(AstExprCall* call)`.
use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::records::cfg_builder::CfgBuilder;

impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `call` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn lower_expr_ast_expr_call(&mut self, call: *mut AstExprCall) {
    unsafe {
      // if (tryLowerAssertion(call)) return;
      if self.try_lower_assertion(call) {
        return;
      }

      // lowerExpr(call->func);
      let func = (*call).func;
      self.lower_expr_ast_expr(func);

      // for (size_t i = 0; i < call->args.size; i++) lowerExpr(call->args.data[i]);
      let args = (*call).args;
      for &arg in args.iter() {
        self.lower_expr_ast_expr(arg);
      }
    }
  }
}
