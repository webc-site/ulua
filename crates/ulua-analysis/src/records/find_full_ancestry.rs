use alloc::vec::Vec;
use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat_function::AstStatFunction, ast_type::AstType,
    ast_visitor::AstVisitor, position::Position,
  },
  visit::ast_expr_visit_ref,
};
#[derive(Debug, Clone)]
pub struct FindFullAncestry {
  pub(crate) nodes: Vec<*mut AstNode>,
  pub(crate) pos: Position,
  pub(crate) document_end: Position,
  pub(crate) include_types: bool,
}

impl AstVisitor for FindFullAncestry {
  fn visit_node(&mut self, node: &mut AstNode) -> bool {
    if node.location.contains(self.pos) {
      self.nodes.push(from_mut(node));
      return true;
    }

    if node.location.end == self.document_end && self.pos >= self.document_end {
      self.nodes.push(from_mut(node));
      return true;
    }

    false
  }

  fn visit_type(&mut self, node: &mut AstType) -> bool {
    if self.include_types {
      self.visit_node(&mut node.base)
    } else {
      false
    }
  }

  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_node(&mut node.base.base);

    // name/func 已句柄化为 Node：`.get()` 即安全只读视图（非空+存活由句柄契约
    // 承载），写穿分发落 ast_expr_visit_ref 引用形态，全链路无裸指针。
    let name_ref = node.name.get();
    let func_ref = node.func.get();

    if name_ref.base.location.contains(self.pos) {
      ast_expr_visit_ref(node.name.get_mut(), self);
    } else if func_ref.base.base.location.contains(self.pos) {
      ast_expr_visit_ref(node.func.cast::<AstExpr>().get_mut(), self);
    }

    false
  }
}
