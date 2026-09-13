use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::{
  records::{ast_expr_local::AstExprLocal, ast_node::AstNode, ast_stat_assign::AstStatAssign},
  rtti::ast_node_as,
};

use crate::records::require_tracer::RequireTracer;
impl RequireTracer {
  pub fn visit_ast_stat_assign(&mut self, node: *mut c_void) -> bool {
    let stat = node as *mut AstStatAssign;
    let stat_ref = unsafe { &*stat };

    for &var in stat_ref.vars.as_slice() {
      let expr_local = unsafe { ast_node_as::<AstExprLocal>(var as *mut AstNode) };

      if !expr_local.is_null() {
        let local = unsafe { (*expr_local).local };
        self.locals.try_insert(local, null_mut());
      }
    }

    true
  }
}
