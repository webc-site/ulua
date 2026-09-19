//! Source: `Analysis/src/ControlFlowGraph.cpp:258-279` (hand-ported)
//! C++ `bool CFGBuilder::tryLowerAssertion(AstExprCall* call)`.
//! 把 `assert(...)` 调用下推为断言精化：逐个下推实参以登记读用，并对首个实参
//! 解析出精化树后发射 Refine 指令。C++ `bool` -> Rust `bool`（true 表示已下推）。
use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal, ast_node::AstNode},
  rtti::ast_node_as,
};

use crate::records::cfg_builder::CfgBuilder;

impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `call` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn try_lower_assertion(&mut self, call: *mut AstExprCall) -> bool {
    unsafe {
      // if (call->args.size == 0) return false;
      let args = (*call).args;
      if args.is_empty() {
        return false;
      }

      // auto global = call->func->as<AstExprGlobal>();
      // if (!global || global->name != "assert") return false;
      let func = (*call).func as *mut AstNode;
      let global = ast_node_as::<AstExprGlobal>(func);
      if global.is_null() || (*global).name != "assert" {
        return false;
      }

      // AstExpr* cond = call->args.data[0];
      let cond = args.as_slice()[0];
      for (i, &arg) in args.iter().enumerate() {
        self.lower_expr_ast_expr(arg);
        // if (i == 0) { if (auto ref = resolveCondition(cond))
        //     emitRefineInstruction(currentBlock, *ref); }
        if i == 0
          && let Some(refinement) = self.resolve_condition(cond)
        {
          let current_block = self.current_block;
          self.emit_refine_instruction(current_block, refinement);
        }
      }

      true
    }
  }
}
