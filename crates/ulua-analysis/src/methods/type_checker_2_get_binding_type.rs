use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal},
  rtti::ast_node_as,
};

use crate::{
  records::{symbol::Symbol, type_checker_2::TypeChecker2},
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  pub fn get_binding_type(&mut self, expr: *mut AstExpr) -> Option<TypeId> {
    let local_expr = unsafe { ast_node_as::<AstExprLocal>(expr as *mut _) };
    if !local_expr.is_null() {
      let s = self.stack.last().copied()?;
      let sym = Symbol::from_local(unsafe { (*local_expr).local });
      return unsafe { (*s).lookup_symbol(sym) };
    }

    let global_expr = unsafe { ast_node_as::<AstExprGlobal>(expr as *mut _) };
    if !global_expr.is_null() {
      let s = self.stack.last().copied()?;
      let sym = Symbol::from_global(unsafe { (*global_expr).name });
      return unsafe { (*s).lookup_symbol(sym) };
    }

    None
  }
}
