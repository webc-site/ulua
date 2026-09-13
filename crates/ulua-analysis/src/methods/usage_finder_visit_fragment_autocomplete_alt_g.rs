use ulua_ast::records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal};

use crate::records::{symbol::Symbol, usage_finder::UsageFinder};

impl UsageFinder {
  /// # Safety
  /// 调用方须保证 `global` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  // C++ `bool UsageFinder::visit(AstExprGlobal* global)` (FragmentAutocomplete.cpp:641-647):
  //   globalDefsToPrePopulate.emplace_back(global->name, dfg->getDef(global));
  //   auto def = dfg->getDef(global);
  //   symbolsToRefine.emplace_back(def, Symbol(global->name));
  //   return true;
  pub(crate) fn visit_ast_expr_global(&mut self, global: *mut AstExprGlobal) -> bool {
    let dfg = unsafe { &*self.dfg };
    let name = unsafe { (*global).name };
    let def = dfg.get_def(global as *const AstExpr);

    self.global_defs_to_pre_populate.push((name, def));
    self
      .symbols_to_refine
      .push((def, Symbol::from_global(name)));

    true
  }
}
