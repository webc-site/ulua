use core::ffi::c_void;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_group::AstExprGroup, ast_node::AstNode,
    ast_visitor::AstVisitor,
  },
  rtti::ast_node_is,
  visit::ast_expr_visit,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::is_literal::is_literal,
  records::blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor,
  type_aliases::type_id::TypeId,
};
impl AstVisitor for BlockedTypeInLiteralVisitor {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_node(node as *mut AstNode)
  }

  fn visit_expr(&mut self, node: *mut c_void) -> bool {
    unsafe { self.visit_ast_expr(node as *mut AstExpr) }
  }
}

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn find_blocked_arg_types_in(
  expr: *mut AstExprCall,
  ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
) -> Vec<TypeId> {
  let mut to_block: Vec<TypeId> = Vec::new();
  let mut v = BlockedTypeInLiteralVisitor {
    ast_types,
    to_block: &mut to_block as *mut Vec<TypeId>,
  };
  unsafe {
    for &arg in (*expr).args.iter() {
      if is_literal(arg as *const AstExpr) || ast_node_is::<AstExprGroup>(&(*arg).base) {
        ast_expr_visit(arg, &mut v);
      }
    }
  }
  to_block
}
