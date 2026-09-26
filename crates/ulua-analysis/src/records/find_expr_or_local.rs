use core::ptr::{from_mut, null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_local::AstLocal,
    ast_stat_block::AstStatBlock, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
    ast_visitor::AstVisitor, location::Location, position::Position,
  },
  visit,
};

use crate::records::expr_or_local::ExprOrLocal;
#[derive(Debug, Clone)]
pub struct FindExprOrLocal {
  pub(crate) pos: Position,
  pub(crate) result: ExprOrLocal,
}

impl FindExprOrLocal {
  pub fn new(pos: Position) -> Self {
    Self {
      pos,
      result: ExprOrLocal {
        expr: null_mut(),
        local: null_mut(),
      },
    }
  }

  pub(crate) fn is_closer_match(&self, new_location: Location) -> bool {
    let current = self.result.get_location();
    new_location.contains(self.pos)
      && (current.is_none() || current.is_some_and(|c| c.encloses(&new_location)))
  }

  fn visit_local(&mut self, local: *mut AstLocal) -> bool {
    // SAFETY: `local` 来自遍历中的存活 AST 节点字段。
    let location = unsafe { &*local }.location;
    if self.is_closer_match(location) {
      self.result.set_local(local);
      true
    } else {
      false
    }
  }
}

impl AstVisitor for FindExprOrLocal {
  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    for stat in node.body.iter_nodes() {
      let stat_ref = stat.get();
      if stat_ref.base.location.end <= self.pos {
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

  fn visit_expr(&mut self, node: &mut AstExpr) -> bool {
    if self.is_closer_match(node.base.location) {
      self.result.set_expr(from_mut(node));
      true
    } else {
      false
    }
  }

  fn visit_stat_local_function(&mut self, node: &mut AstStatLocalFunction) -> bool {
    self.visit_local(node.name.as_ptr());
    true
  }

  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    for &local in node.vars.as_slice() {
      self.visit_local(local);
    }
    true
  }

  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    for local in node.args.iter_nodes() {
      self.visit_local(local.as_ptr());
    }
    self.visit_expr(&mut node.base)
  }

  fn visit_stat_for(&mut self, node: &mut AstStatFor) -> bool {
    // var 已句柄化为 Node（非空由类型层承载），as_ptr 桥交指针形态 visit_local。
    self.visit_local(node.var.as_ptr());
    true
  }

  fn visit_stat_for_in(&mut self, node: &mut AstStatForIn) -> bool {
    for &local in node.vars.as_slice() {
      self.visit_local(local);
    }
    true
  }
}
