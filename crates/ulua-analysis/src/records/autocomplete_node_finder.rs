use alloc::vec::Vec;
use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_expr_index_name::AstExprIndexName,
    ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_type::AstType,
    ast_type_error::AstTypeError, ast_type_pack::AstTypePack, ast_visitor::AstVisitor,
    position::Position,
  },
  rtti::ast_node_is_ptr,
};
#[derive(Debug, Clone)]
pub struct AutocompleteNodeFinder {
  pub(crate) pos: Position,
  pub(crate) ancestry: Vec<*mut AstNode>,
}

impl AutocompleteNodeFinder {
  pub fn new(pos: Position) -> Self {
    Self {
      pos,
      ancestry: Vec::new(),
    }
  }
}

impl AstVisitor for AutocompleteNodeFinder {
  fn visit_expr(&mut self, node: &mut AstExpr) -> bool {
    let loc = node.base.location;
    if loc.begin <= self.pos && self.pos <= loc.end && loc.begin != loc.end {
      self.ancestry.push(from_mut(&mut node.base));
      return true;
    }
    false
  }

  fn visit_stat(&mut self, node: &mut AstStat) -> bool {
    let loc = node.base.location;
    let has_semicolon = node.has_semicolon;
    if loc.begin < self.pos
      && (if has_semicolon {
        self.pos < loc.end
      } else {
        self.pos <= loc.end
      })
    {
      self.ancestry.push(from_mut(&mut node.base));
      return true;
    }
    false
  }

  fn visit_type(&mut self, node: &mut AstType) -> bool {
    let loc = node.base.location;
    if loc.begin < self.pos && self.pos <= loc.end {
      self.ancestry.push(from_mut(&mut node.base));
      return true;
    }
    false
  }

  fn visit_type_error(&mut self, node: &mut AstTypeError) -> bool {
    if node.is_missing && node.base.base.location.contains_closed(self.pos) {
      self.ancestry.push(from_mut(&mut node.base.base));
      return true;
    }
    false
  }

  fn visit_type_pack(&mut self, _node: &mut AstTypePack) -> bool {
    true
  }

  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    if self.ancestry.is_empty() {
      self.ancestry.push(from_mut(&mut node.base.base));
      return true;
    }

    // SAFETY: ancestry 中保存的是遍历期间存活的 AST 节点指针。
    unsafe {
      // 上方 `is_empty()` 分支已 push 并返回，此处 ancestry 必非空，
      // last() 命中 Some 由该早退蕴含。
      let last = *self
        .ancestry
        .last()
        .expect("上方 is_empty 早退蕴含 ancestry 非空");
      if ast_node_is_ptr::<AstExprIndexName>(last) {
        return false;
      }
      if ast_node_is_ptr::<AstTypeError>(last) {
        return false;
      }

      let loc = node.base.base.location;
      if loc.begin == self.pos {
        if (*last).as_expr_const().is_some() && !ast_node_is_ptr::<AstExprFunction>(last) {
          return false;
        }
        if (*last).as_type_const().is_some() {
          return false;
        }
      }

      if loc.begin <= self.pos && self.pos <= loc.end {
        self.ancestry.push(from_mut(&mut node.base.base));
        return true;
      }
    }
    false
  }
}
