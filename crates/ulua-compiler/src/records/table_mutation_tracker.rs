use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, ast_expr_call::AstExprCall,
    ast_expr_table::AstExprTable, ast_local::AstLocal, ast_stat_assign::AstStatAssign,
    ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_local::AstStatLocal,
    ast_stat_return::AstStatReturn, ast_visitor::AstVisitor,
  },
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::node::Node;

#[derive(Debug)]
pub(crate) struct TableMutationTracker {
  pub(crate) escaped: DenseHashSet<Node<AstLocal>>,
}

impl TableMutationTracker {
  pub fn new() -> Self {
    Self {
      escaped: DenseHashSet::default(),
    }
  }

  /// 标记 expr 触及的所有 local 为逃逸。包装层（group / assertion /
  /// instantiate）沿内层继续，条件短路（if-else、And/Or）两支都算。
  fn mark_escaped(&mut self, mut expr: Node<AstExpr>) {
    loop {
      match expr.as_expr_ref() {
        AstExprRef::Local(local) => {
          self.escaped.insert(local.local.into());
          return;
        }
        AstExprRef::Group(group) => {
          expr = group.expr.into();
        }
        AstExprRef::TypeAssertion(assertion) => {
          expr = assertion.expr.into();
        }
        AstExprRef::Instantiate(inst) => {
          expr = inst.expr.into();
        }
        AstExprRef::IfElse(if_else) => {
          self.mark_escaped(if_else.true_expr.into());
          expr = if_else.false_expr.into();
        }
        AstExprRef::Binary(bin)
          if bin.op == AstExprBinaryOp::And || bin.op == AstExprBinaryOp::Or =>
        {
          self.mark_escaped(bin.left.into());
          expr = bin.right.into();
        }
        _ => return,
      }
    }
  }

  /// 批量标记：`AstArray<*mut AstExpr>` 槽位序列整体按逃逸处理（各 visit 臂
  /// 同构循环的单点收口）。
  fn mark_escaped_all(&mut self, exprs: &[*mut AstExpr]) {
    for &expr in exprs {
      self.mark_escaped(expr.into());
    }
  }

  fn mark_escaped_table_index(&mut self, expr: Node<AstExpr>, is_lvalue: bool) {
    match expr.as_expr_ref() {
      AstExprRef::IndexName(idx_name) => {
        self.mark_escaped(idx_name.expr.into());
      }
      AstExprRef::IndexExpr(idx_expr) => {
        self.mark_escaped(idx_expr.expr.into());
        if is_lvalue {
          self.mark_escaped(idx_expr.index.into());
        }
      }
      _ => {}
    }
  }
}

impl AstVisitor for TableMutationTracker {
  fn visit_expr_call(&mut self, node: &mut AstExprCall) -> bool {
    self.mark_escaped_all(node.args.as_slice());

    if node.self_ {
      self.mark_escaped_table_index(node.func.into(), false);
    }

    true
  }

  fn visit_expr_table(&mut self, node: &mut AstExprTable) -> bool {
    for item in node.items.iter() {
      if !item.key.is_null() {
        self.mark_escaped(item.key.into());
      }
      self.mark_escaped(item.value.into());
    }
    true
  }

  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    // cpp `min(vars.size, values.size)`：初值只记到与变量数等长的前缀
    let paired = node.values.size.min(node.vars.size);
    self.mark_escaped_all(&node.values.as_slice()[..paired]);
    true
  }

  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    self.mark_escaped_all(node.values.as_slice());
    for lhs in node.vars.iter() {
      self.mark_escaped_table_index((*lhs).into(), true);
    }
    true
  }

  fn visit_stat_compound_assign(&mut self, node: &mut AstStatCompoundAssign) -> bool {
    self.mark_escaped_table_index(node.var.into(), true);
    true
  }

  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.mark_escaped_table_index(node.name.into(), true);
    true
  }

  fn visit_stat_for_in(&mut self, node: &mut AstStatForIn) -> bool {
    self.mark_escaped_all(node.values.as_slice());
    true
  }

  fn visit_stat_return(&mut self, node: &mut AstStatReturn) -> bool {
    self.mark_escaped_all(node.list.as_slice());
    true
  }
}
