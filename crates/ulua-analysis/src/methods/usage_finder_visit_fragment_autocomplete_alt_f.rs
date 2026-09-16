use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_local::AstExprLocal, ast_node::AstNode},
  rtti::ast_node_as,
};

use crate::records::{def::Def, symbol::Symbol, usage_finder::UsageFinder};

impl UsageFinder {
  pub fn visit_ast_expr(&mut self, expr: *mut AstExpr) -> bool {
    let dfg = unsafe { &*self.dfg };

    if let Some(opt) = dfg.get_def_optional(expr) {
      self.mentioned_defs.insert(opt);
    }

    let ref_ = dfg.get_refinement_key(expr);
    if !ref_.is_null() {
      self
        .mentioned_defs
        .insert(unsafe { (*ref_).def() } as *const Def);
    }

    let local = unsafe { ast_node_as::<AstExprLocal>(expr as *mut AstNode) };
    if !local.is_null() {
      let def = dfg.get_def(local as *const AstExpr);
      let ast_local = unsafe { (*local).local };
      self.local_bindings_referenced.push((def, ast_local));
      self
        .symbols_to_refine
        .push((def, Symbol::from_local(ast_local)));
    }

    true
  }
}
