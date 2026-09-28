use core::ptr::{from_mut, null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat_block::AstStatBlock,
    ast_stat_function::AstStatFunction, ast_visitor::AstVisitor, position::Position,
  },
  visit,
};
#[derive(Debug, Clone)]
pub struct FindNode {
  pub(crate) pos: Position,
  pub(crate) document_end: Position,
  pub(crate) best: *mut AstNode,
}

impl FindNode {
  pub fn new(pos: Position, document_end: Position) -> Self {
    Self {
      pos,
      document_end,
      best: null_mut(),
    }
  }
}

impl AstVisitor for FindNode {
  fn visit_node(&mut self, node: &mut AstNode) -> bool {
    if node.location.contains(self.pos) {
      self.best = from_mut(node);
      return true;
    }

    if node.location.end == self.document_end && self.pos >= self.document_end {
      self.best = from_mut(node);
      return true;
    }

    false
  }

  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_node(&mut node.base.base);
    // name/func 已句柄化为 Node（非空+存活由句柄契约承载），基类视图取
    // `.get()`、写穿分发落 ast_expr_visit_ref 引用形态，不再有裸指针解引用。
    let name_ref = node.name.get();
    let func_ref = node.func.get();
    if name_ref.base.location.contains(self.pos) {
      visit::ast_expr_visit_ref(node.name.get_mut(), self);
    } else if func_ref.base.base.location.contains(self.pos) {
      visit::ast_expr_visit_ref(node.func.cast::<AstExpr>().get_mut(), self);
    }
    false
  }

  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    self.visit_node(&mut node.base.base);
    for stat in node.body.iter_nodes() {
      let stat_ref = stat.get();
      if stat_ref.base.location.end < self.pos {
        continue;
      }
      if stat_ref.base.location.begin > self.pos {
        break;
      }
      // SAFETY: 同上，委托 ulua-ast 的遍历分发。
      unsafe { visit::ast_stat_visit(stat.as_ptr(), self) };
    }
    false
  }
}
