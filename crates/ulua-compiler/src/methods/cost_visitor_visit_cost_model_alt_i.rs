use core::ffi::c_void;

use ulua_ast::{
  records::{
    ast_expr_local::AstExprLocal, ast_node::AstNode,
    ast_stat_compound_assign::AstStatCompoundAssign,
  },
  rtti::ast_node_is,
};

use crate::records::{cost::Cost, cost_visitor::CostVisitor};

impl CostVisitor {
  pub fn visit_ast_stat_compound_assign(&mut self, node: *mut c_void) -> bool {
    if node.is_null() {
      return true;
    }

    unsafe {
      let node = &*(node as *mut AstStatCompoundAssign);

      // assign(node->var)
      self.assign(node.var);

      // if lhs is not a local, setting it requires an extra table operation
      let is_local = ast_node_is::<AstExprLocal>(&*(node.var as *mut AstNode));
      let cost_increment = if is_local { 1 } else { 2 };
      self
        .result
        .operator_add_assign(&Cost::new(cost_increment, 0));
    }

    true
  }
}
