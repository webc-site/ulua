//! Source: `Analysis/src/ControlFlowGraph.cpp:258-279` (hand-ported)
//! C++ `bool CFGBuilder::tryLowerAssertion(AstExprCall* call)`.
//! 把 `assert(...)` 调用下推为断言精化：逐个下推实参以登记读用，并对首个实参
//! 解析出精化树后发射 Refine 指令。C++ `bool` -> Rust `bool`（true 表示已下推）。
use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal},
  rtti::ast_node_try_as_ptr,
};

use crate::records::cfg_builder::CfgBuilder;

impl CfgBuilder {
  /// # Safety
  /// `call` 须指向 `CfgBuilder` 在本次 CFG 构建期间拥有的存活节点：非空、对齐，地址在 builder
  /// 存续期内不移动；本函数只读 call 并写入 self 的指令流，调用方（builder 自身或其遍历）单线程独占写，返回后所有权仍归 builder。
  /// 对应 C++ `bool CFGBuilder::tryLowerAssertion(AstExprCall* call)` (`cpp/Analysis/src/ControlFlowGraph.cpp:258`)。
  pub unsafe fn try_lower_assertion(&mut self, call: *mut AstExprCall) -> bool {
    unsafe {
      // if (call->args.size == 0) return false;
      let args = (*call).args;
      if args.is_empty() {
        return false;
      }

      // auto global = call->func->as<AstExprGlobal>();
      // if (!global || global->name != "assert") return false;
      // cpp `call->func->as<AstExprGlobal>()` 判型+判空折叠为 Option 门面。
      let Some(global) = ast_node_try_as_ptr::<AstExprGlobal>((*call).func) else {
        return false;
      };
      if global.name != "assert" {
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
