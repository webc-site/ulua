use alloc::vec::Vec;
use core::ffi::c_void;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_expr_index_name::AstExprIndexName,
  ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_type::AstType,
  ast_type_error::AstTypeError, ast_visitor::AstVisitor, position::Position,
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
  fn visit_node(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_expr(&mut self, node: *mut c_void) -> bool {
    let expr = node as *mut AstExpr;
    unsafe {
      let loc = (*expr).base.location;
      if loc.begin <= self.pos && self.pos <= loc.end && loc.begin != loc.end {
        self.ancestry.push(expr as *mut AstNode);
        return true;
      }
    }
    false
  }

  fn visit_stat(&mut self, node: *mut c_void) -> bool {
    let stat = node as *mut AstStat;
    unsafe {
      let loc = (*stat).base.location;
      let has_semicolon = (*stat).has_semicolon;
      if loc.begin < self.pos
        && (if has_semicolon {
          self.pos < loc.end
        } else {
          self.pos <= loc.end
        })
      {
        self.ancestry.push(stat as *mut AstNode);
        return true;
      }
    }
    false
  }

  fn visit_type(&mut self, node: *mut c_void) -> bool {
    let ty = node as *mut AstType;
    unsafe {
      let loc = (*ty).base.location;
      if loc.begin < self.pos && self.pos <= loc.end {
        self.ancestry.push(ty as *mut AstNode);
        return true;
      }
    }
    false
  }

  fn visit_type_error(&mut self, node: *mut c_void) -> bool {
    let ty = node as *mut AstTypeError;
    unsafe {
      if (*ty).is_missing && (*ty).base.base.location.contains_closed(self.pos) {
        self.ancestry.push(ty as *mut AstNode);
        return true;
      }
    }
    false
  }

  fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    let block = node as *mut AstStatBlock;
    if self.ancestry.is_empty() {
      self.ancestry.push(block as *mut AstNode);
      return true;
    }

    unsafe {
      let last = *self.ancestry.last().unwrap();
      if (*last).is::<AstExprIndexName>() {
        return false;
      }
      if (*last).is::<AstTypeError>() {
        return false;
      }

      let loc = (*block).base.base.location;
      if loc.begin == self.pos {
        if let Some(_expr) = (*last).as_expr_const().as_ref()
          && !(*last).is::<AstExprFunction>()
        {
          return false;
        }
        if (*last).as_type().as_ref().is_some() {
          return false;
        }
      }

      if loc.begin <= self.pos && self.pos <= loc.end {
        self.ancestry.push(block as *mut AstNode);
        return true;
      }
    }
    false
  }
}
