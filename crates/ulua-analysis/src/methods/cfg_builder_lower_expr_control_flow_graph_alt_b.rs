//! Source: `Analysis/src/ControlFlowGraph.cpp:362-366` (hand-ported)
//! C++ `void CFGBuilder::lowerExpr(AstExprLocal* local)`.
use ulua_ast::records::{ast_expr::AstExpr, ast_expr_local::AstExprLocal};

use crate::{
  records::{cfg_builder::CfgBuilder, symbol::Symbol},
  type_aliases::def_id_control_flow_graph::DefId,
};

impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `local` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn lower_expr_ast_expr_local(&mut self, local: *mut AstExprLocal) {
    unsafe {
      // C++:
      //   DefId def = readVariable(currentBlock, Symbol(local->local));
      //   cfg->useDefs[local] = def;
      let sym = Symbol::from_local((*local).local);
      let def: DefId = self.read_variable(self.current_block, sym);
      // useDefs is keyed by `AstExpr*`; `AstExprLocal*` upcasts to it.
      let key = local as *mut AstExpr;
      *self.cfg.as_mut().unwrap().use_defs.get_or_insert(key) = def;
    }
  }
}
