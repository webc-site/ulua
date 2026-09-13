use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_call::AstExprCall,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate,
    ast_expr_local::AstExprLocal,
    ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_local::AstLocal,
    ast_node::AstNode,
    ast_stat_assign::AstStatAssign,
    ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction,
    ast_stat_local::AstStatLocal,
    ast_stat_return::AstStatReturn,
    ast_visitor::AstVisitor,
  },
  rtti::ast_node_as,
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::records::variable::Variable;

#[derive(Debug)]
pub struct TableMutationTracker {
  pub(crate) escaped: DenseHashSet<*mut AstLocal>,
}

impl TableMutationTracker {
  pub fn new(_variables: &DenseHashMap<*mut AstLocal, Variable>) -> Self {
    Self {
      escaped: DenseHashSet::new(null_mut()),
    }
  }

  pub fn mark_escaped(&mut self, mut expr: *mut AstExpr) {
    loop {
      if expr.is_null() {
        return;
      }
      unsafe {
        let node_ptr = expr as *mut AstNode;

        let local = ast_node_as::<AstExprLocal>(node_ptr);
        if !local.is_null() {
          self.escaped.insert((*local).local);
          return;
        }

        let group = ast_node_as::<AstExprGroup>(node_ptr);
        if !group.is_null() {
          expr = (*group).expr;
          continue;
        }

        let assertion = ast_node_as::<AstExprTypeAssertion>(node_ptr);
        if !assertion.is_null() {
          expr = (*assertion).expr;
          continue;
        }

        let inst = ast_node_as::<AstExprInstantiate>(node_ptr);
        if !inst.is_null() {
          expr = (*inst).expr;
          continue;
        }

        let if_else = ast_node_as::<AstExprIfElse>(node_ptr);
        if !if_else.is_null() {
          self.mark_escaped((*if_else).true_expr);
          expr = (*if_else).false_expr;
          continue;
        }

        let bin = ast_node_as::<AstExprBinary>(node_ptr);
        if !bin.is_null() {
          if (*bin).op == AstExprBinaryOp::And || (*bin).op == AstExprBinaryOp::Or {
            self.mark_escaped((*bin).left);
            expr = (*bin).right;
            continue;
          } else {
            return;
          }
        }

        return;
      }
    }
  }

  pub fn mark_escaped_table_index(&mut self, expr: *mut AstExpr, is_lvalue: bool) {
    if expr.is_null() {
      return;
    }
    unsafe {
      let node_ptr = expr as *mut AstNode;

      let idx_name = ast_node_as::<AstExprIndexName>(node_ptr);
      if !idx_name.is_null() {
        self.mark_escaped((*idx_name).expr);
        return;
      }

      let idx_expr = ast_node_as::<AstExprIndexExpr>(node_ptr);
      if !idx_expr.is_null() {
        self.mark_escaped((*idx_expr).expr);
        if is_lvalue {
          self.mark_escaped((*idx_expr).index);
        }
      }
    }
  }
}

impl AstVisitor for TableMutationTracker {
  fn visit_expr_call(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstExprCall;
    unsafe {
      for arg in (*node).args.iter() {
        self.mark_escaped(*arg);
      }

      if (*node).self_ {
        self.mark_escaped_table_index((*node).func, false);
      }
    }

    true
  }

  fn visit_expr_table(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstExprTable;
    unsafe {
      for item in (*node).items.iter() {
        if !item.key.is_null() {
          self.mark_escaped(item.key);
        }
        self.mark_escaped(item.value);
      }
    }
    true
  }

  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatLocal;
    unsafe {
      for (value, _var) in (*node).values.iter().zip((*node).vars.iter()) {
        self.mark_escaped(*value);
      }
    }
    true
  }

  fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatAssign;
    unsafe {
      for rhs in (*node).values.iter() {
        self.mark_escaped(*rhs);
      }
      for lhs in (*node).vars.iter() {
        self.mark_escaped_table_index(*lhs, true);
      }
    }
    true
  }

  fn visit_stat_compound_assign(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatCompoundAssign;
    unsafe {
      self.mark_escaped_table_index((*node).var, true);
    }
    true
  }

  fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatFunction;
    unsafe {
      self.mark_escaped_table_index((*node).name, true);
    }
    true
  }

  fn visit_stat_for_in(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatForIn;
    unsafe {
      for expr in (*node).values.iter() {
        self.mark_escaped(*expr);
      }
    }
    true
  }

  fn visit_stat_return(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatReturn;
    unsafe {
      for expr in (*node).list.iter() {
        self.mark_escaped(*expr);
      }
    }
    true
  }
}
