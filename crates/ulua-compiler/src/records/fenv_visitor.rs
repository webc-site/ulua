use core::ffi::c_void;

use ulua_ast::{
  records::{ast_expr_global::AstExprGlobal, ast_node::AstNode, ast_visitor::AstVisitor},
  rtti::ast_node_as,
};

#[derive(Debug)]
pub struct FenvVisitor<'a> {
  pub(crate) getfenv_used: &'a mut bool,
  pub(crate) setfenv_used: &'a mut bool,
}

impl<'a> AstVisitor for FenvVisitor<'a> {
  fn visit_expr_global(&mut self, node: *mut c_void) -> bool {
    let node = unsafe { ast_node_as::<AstExprGlobal>(node as *mut AstNode) };
    if let Some(node) = unsafe { node.as_ref() } {
      let name = node.name.as_bytes();
      if name == b"getfenv" {
        *self.getfenv_used = true;
      }
      if name == b"setfenv" {
        *self.setfenv_used = true;
      }
    }
    false
  }
}
