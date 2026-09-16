use core::ffi::c_void;

use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
    ast_stat_assign::AstStatAssign, ast_stat_local::AstStatLocal, location::Location,
  },
  rtti::ast_node_as,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  records::lint_unbalanced_assignment::LintUnbalancedAssignment,
};
impl LintUnbalancedAssignment {
  pub fn assign(&mut self, vars: usize, values: &AstArray<*mut AstExpr>, location: Location) {
    let vals = values.as_slice();
    if vars != vals.len() && !vals.is_empty() {
      let last = vals[vals.len() - 1];

      if vars < vals.len() {
        let msg = format!(
          "Assigning {} values to {} variables leaves some values unused",
          vals.len(),
          vars
        );
        emit_warning(
          unsafe { &mut *self.context },
          Code::UnbalancedAssignment,
          location,
          format_args!("{}", msg),
        );
      } else if !unsafe { ast_node_as::<AstExprCall>(last as *mut AstNode) }.is_null()
        || !unsafe { ast_node_as::<AstExprVarargs>(last as *mut AstNode) }.is_null()
        || !unsafe { ast_node_as::<AstExprConstantNil>(last as *mut AstNode) }.is_null()
      {
        // we don't know how many values the last expression returns
        // or last expression is nil which explicitly silences the nil-init warning
      } else {
        let msg = format!(
          "Assigning {} values to {} variables initializes extra variables with nil; add 'nil' to value list to silence",
          vals.len(),
          vars
        );
        emit_warning(
          unsafe { &mut *self.context },
          Code::UnbalancedAssignment,
          location,
          format_args!("{}", msg),
        );
      }
    }
  }

  pub fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatLocal;
    unsafe {
      self.assign(
        (*node).vars.len(),
        &(*node).values,
        (*node).base.base.location,
      );
    }

    true
  }

  pub fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatAssign;
    unsafe {
      self.assign(
        (*node).vars.len(),
        &(*node).values,
        (*node).base.base.location,
      );
    }

    true
  }
}
