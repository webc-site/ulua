use core::ffi::c_void;

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal,
    ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs,
    ast_local::AstLocal,
    ast_node::AstNode,
    ast_stat::AstStat,
    ast_stat_assign::AstStatAssign,
    ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction,
    ast_stat_local::AstStatLocal,
    ast_stat_return::AstStatReturn,
    ast_visitor::AstVisitor,
  },
  rtti::{ast_node_as, ast_node_is},
  visit::ast_stat_visit,
};
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{enums::table_constant_kind::TableConstantKind, records::variable::Variable};

#[derive(Debug)]
pub struct TableMutationTrackerDeprecated<'a> {
  pub(crate) constant_tables: &'a mut DenseHashMap<*mut AstLocal, TableConstantKind>,
  pub(crate) variables: &'a DenseHashMap<*mut AstLocal, Variable>,
}

impl<'a> TableMutationTrackerDeprecated<'a> {
  pub fn new(
    constant_tables: &'a mut DenseHashMap<*mut AstLocal, TableConstantKind>,
    variables: &'a DenseHashMap<*mut AstLocal, Variable>,
  ) -> Self {
    LUAU_ASSERT!(FFlag::LuauCompilePropagateTableProps2.get());
    Self {
      constant_tables,
      variables,
    }
  }

  pub fn is_non_table_constant(&self, node: *mut AstExpr) -> bool {
    unsafe {
      if let Some(expr) = ast_node_as::<AstExprGroup>(node as *mut AstNode).as_mut() {
        return self.is_non_table_constant(expr.expr);
      }

      // C++ 对 Nil/Bool/Number/Integer/String 各写一个 `return true` 分支；
      // 节点类型互斥，合并为等价的单一条件
      if ast_node_is::<AstExprConstantNil>(node as *mut AstNode)
        || ast_node_is::<AstExprConstantBool>(node as *mut AstNode)
        || ast_node_is::<AstExprConstantNumber>(node as *mut AstNode)
        || ast_node_is::<AstExprConstantInteger>(node as *mut AstNode)
        || ast_node_is::<AstExprConstantString>(node as *mut AstNode)
      {
        return true;
      } else if ast_node_is::<AstExprLocal>(node as *mut AstNode) {
        let expr = &*(node as *mut AstExprLocal);
        if let Some(kind) = self.constant_tables.find(&expr.local) {
          return *kind == TableConstantKind::ConstantOther;
        }
        return false;
      } else if ast_node_is::<AstExprGlobal>(node as *mut AstNode)
        || ast_node_is::<AstExprVarargs>(node as *mut AstNode)
        || ast_node_is::<AstExprCall>(node as *mut AstNode)
      {
        return false;
      } else if ast_node_is::<AstExprIndexName>(node as *mut AstNode) {
        let expr = &*(node as *mut AstExprIndexName);
        let local = ast_node_as::<AstExprLocal>(expr.expr as *mut AstNode);
        if local.is_null() {
          return false;
        }
        let local_ref = &*local;
        if let Some(kind) = self.constant_tables.find(&local_ref.local) {
          return *kind == TableConstantKind::ConstantTable;
        }
        return false;
      } else if ast_node_is::<AstExprIndexExpr>(node as *mut AstNode) {
        let expr = &*(node as *mut AstExprIndexExpr);
        let local = ast_node_as::<AstExprLocal>(expr.expr as *mut AstNode);
        if local.is_null() {
          return false;
        }
        let local_ref = &*local;
        if let Some(kind) = self.constant_tables.find(&local_ref.local) {
          return *kind == TableConstantKind::ConstantTable
            && self.is_non_table_constant(expr.index);
        }
        return false;
      } else if ast_node_is::<AstExprFunction>(node as *mut AstNode)
        || ast_node_is::<AstExprTable>(node as *mut AstNode)
      {
        // 函数与表字面量都不是非表常量
        return false;
      } else if ast_node_is::<AstExprUnary>(node as *mut AstNode) {
        let expr = &*(node as *mut AstExprUnary);
        return self.is_non_table_constant(expr.expr);
      } else if ast_node_is::<AstExprBinary>(node as *mut AstNode) {
        let expr = &*(node as *mut AstExprBinary);
        return self.is_non_table_constant(expr.left) && self.is_non_table_constant(expr.right);
      } else if ast_node_is::<AstExprTypeAssertion>(node as *mut AstNode) {
        let expr = &*(node as *mut AstExprTypeAssertion);
        return self.is_non_table_constant(expr.expr);
      } else if ast_node_is::<AstExprIfElse>(node as *mut AstNode) {
        let expr = &*(node as *mut AstExprIfElse);
        return self.is_non_table_constant(expr.condition)
          && self.is_non_table_constant(expr.true_expr)
          && self.is_non_table_constant(expr.false_expr);
      } else if ast_node_is::<AstExprInterpString>(node as *mut AstNode) {
        let expr = &*(node as *mut AstExprInterpString);
        for &expression in expr.expressions.iter() {
          if !self.is_non_table_constant(expression) {
            return false;
          }
        }
        return true;
      } else if ast_node_is::<AstExprInstantiate>(node as *mut AstNode) {
        let expr = &*(node as *mut AstExprInstantiate);
        return self.is_non_table_constant(expr.expr);
      }

      LUAU_ASSERT!(false);
    }
    false
  }

  pub fn is_constant_table_literal(&self, node: *mut AstExpr) -> bool {
    unsafe {
      if let Some(table) = ast_node_as::<AstExprTable>(node as *mut AstNode).as_mut() {
        for item in table.items.iter() {
          if !item.key.is_null() && !self.is_non_table_constant(item.key) {
            return false;
          }
          if !self.is_non_table_constant(item.value) {
            return false;
          }
        }
        return true;
      }

      if let Some(group) = ast_node_as::<AstExprGroup>(node as *mut AstNode).as_mut() {
        return self.is_constant_table_literal(group.expr);
      }

      if let Some(assert) = ast_node_as::<AstExprTypeAssertion>(node as *mut AstNode).as_mut() {
        return self.is_constant_table_literal(assert.expr);
      }

      if let Some(instantiate) = ast_node_as::<AstExprInstantiate>(node as *mut AstNode).as_mut() {
        return self.is_constant_table_literal(instantiate.expr);
      }

      false
    }
  }

  pub fn could_be_table_reference(&self, node: *mut AstExpr) -> bool {
    unsafe {
      if let Some(expr) = ast_node_as::<AstExprGroup>(node as *mut AstNode).as_mut() {
        return self.could_be_table_reference(expr.expr);
      } else if let Some(expr) = ast_node_as::<AstExprTypeAssertion>(node as *mut AstNode).as_mut()
      {
        return self.could_be_table_reference(expr.expr);
      } else if let Some(expr) = ast_node_as::<AstExprInstantiate>(node as *mut AstNode).as_mut() {
        return self.could_be_table_reference(expr.expr);
      } else if let Some(expr) = ast_node_as::<AstExprIfElse>(node as *mut AstNode).as_mut() {
        return self.could_be_table_reference(expr.true_expr)
          || self.could_be_table_reference(expr.false_expr);
      } else if let Some(bin_expr) = ast_node_as::<AstExprBinary>(node as *mut AstNode).as_mut()
        && (bin_expr.op == AstExprBinaryOp::And || bin_expr.op == AstExprBinaryOp::Or)
      {
        return self.could_be_table_reference(bin_expr.left)
          || self.could_be_table_reference(bin_expr.right);
      }

      ast_node_is::<AstExprLocal>(node as *mut AstNode)
    }
  }

  pub fn observe_mutations(&mut self, node: *mut AstExpr, could_mutate_table: bool) {
    unsafe {
      if let Some(expr) = ast_node_as::<AstExprGroup>(node as *mut AstNode).as_mut() {
        self.observe_mutations(expr.expr, could_mutate_table);
      // 常量表达式不会引发表变异；C++ 对每种常量各写一个空分支，此处合并
      } else if ast_node_is::<AstExprConstantNil>(node as *mut AstNode)
        || ast_node_is::<AstExprConstantBool>(node as *mut AstNode)
        || ast_node_is::<AstExprConstantNumber>(node as *mut AstNode)
        || ast_node_is::<AstExprConstantInteger>(node as *mut AstNode)
        || ast_node_is::<AstExprConstantString>(node as *mut AstNode)
      {
      } else if let Some(expr) = ast_node_as::<AstExprLocal>(node as *mut AstNode).as_mut() {
        let local = expr.local;
        if could_mutate_table && self.constant_tables.contains_key(&local) {
          *self.constant_tables.get_or_insert(local) = TableConstantKind::NotConstant;
        }
      } else if ast_node_is::<AstExprGlobal>(node as *mut AstNode)
        || ast_node_is::<AstExprVarargs>(node as *mut AstNode)
      {
      } else if let Some(expr) = ast_node_as::<AstExprCall>(node as *mut AstNode).as_mut() {
        self.observe_mutations(expr.func, true);
        for &arg in expr.args.iter() {
          let could_mutate = self.could_be_table_reference(arg);
          self.observe_mutations(arg, could_mutate);
        }
      } else if let Some(expr) = ast_node_as::<AstExprIndexName>(node as *mut AstNode).as_mut() {
        self.observe_mutations(expr.expr, could_mutate_table);
      } else if let Some(expr) = ast_node_as::<AstExprIndexExpr>(node as *mut AstNode).as_mut() {
        self.observe_mutations(expr.index, false);
        self.observe_mutations(expr.expr, could_mutate_table);
      } else if let Some(expr) = ast_node_as::<AstExprFunction>(node as *mut AstNode).as_mut() {
        ast_stat_visit(expr.body as *mut AstStat, self);
      } else if let Some(expr) = ast_node_as::<AstExprTable>(node as *mut AstNode).as_mut() {
        for item in expr.items.iter() {
          if !item.key.is_null() {
            self.observe_mutations(item.key, false);
          }
          self.observe_mutations(item.value, self.could_be_table_reference(item.value));
        }
      } else if let Some(expr) = ast_node_as::<AstExprUnary>(node as *mut AstNode).as_mut() {
        self.observe_mutations(expr.expr, false);
      } else if let Some(expr) = ast_node_as::<AstExprBinary>(node as *mut AstNode).as_mut() {
        let short_circuiting = expr.op == AstExprBinaryOp::And || expr.op == AstExprBinaryOp::Or;
        self.observe_mutations(expr.left, short_circuiting);
        self.observe_mutations(expr.right, short_circuiting);
      } else if let Some(expr) = ast_node_as::<AstExprTypeAssertion>(node as *mut AstNode).as_mut()
      {
        self.observe_mutations(expr.expr, could_mutate_table);
      } else if let Some(expr) = ast_node_as::<AstExprIfElse>(node as *mut AstNode).as_mut() {
        self.observe_mutations(expr.condition, false);
        self.observe_mutations(expr.true_expr, could_mutate_table);
        self.observe_mutations(expr.false_expr, could_mutate_table);
      } else if let Some(expr) = ast_node_as::<AstExprInterpString>(node as *mut AstNode).as_mut() {
        for &expression in expr.expressions.iter() {
          self.observe_mutations(expression, false);
        }
      } else if let Some(expr) = ast_node_as::<AstExprInstantiate>(node as *mut AstNode).as_mut() {
        self.observe_mutations(expr.expr, could_mutate_table);
      } else {
        LUAU_ASSERT!(false);
      }
    }
  }
}

impl<'a> AstVisitor for TableMutationTrackerDeprecated<'a> {
  fn visit_expr(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstExpr;
    self.observe_mutations(node, false);
    false
  }

  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let node = &*(node as *mut AstStatLocal);

      // zip 取较短一侧，等价于 C++ 的 `min(vars.size, values.size)` 循环
      for (&local_ptr, &rhs) in node.vars.iter().zip(node.values.iter()) {
        let v = self.variables.find(&local_ptr);
        LUAU_ASSERT!(v.is_some());
        let v = v.unwrap();

        if !v.written {
          if self.is_constant_table_literal(rhs) {
            *self.constant_tables.get_or_insert(local_ptr) = TableConstantKind::ConstantTable;
          } else if self.is_non_table_constant(rhs) {
            *self.constant_tables.get_or_insert(local_ptr) = TableConstantKind::ConstantOther;
          }
        }

        if !self.constant_tables.contains_key(&local_ptr) {
          self.observe_mutations(rhs, self.could_be_table_reference(rhs));
        }
      }

      if node.vars.size < node.values.size {
        for &rhs in node.values.iter().skip(node.vars.size) {
          self.observe_mutations(rhs, false);
        }
      }

      false
    }
  }

  fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let node = &*(node as *mut AstStatAssign);

      for (&_lhs, &rhs) in node.vars.iter().zip(node.values.iter()) {
        self.observe_mutations(rhs, self.could_be_table_reference(rhs));
      }

      if node.values.size > node.vars.size {
        for &rhs in node.values.iter().skip(node.vars.size) {
          self.observe_mutations(rhs, false);
        }
      }

      for &lhs in node.vars.iter() {
        self.observe_mutations(lhs, true);
      }

      false
    }
  }

  fn visit_stat_compound_assign(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let node = &*(node as *mut AstStatCompoundAssign);
      let rhs = node.value;
      self.observe_mutations(rhs, self.could_be_table_reference(rhs));
      self.observe_mutations(node.var, true);
      false
    }
  }

  fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let node = &*(node as *mut AstStatFunction);
      self.observe_mutations(node.func as *mut AstExpr, false);
      self.observe_mutations(node.name, true);
      false
    }
  }

  fn visit_stat_return(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let node = &*(node as *mut AstStatReturn);
      for &expr in node.list.iter() {
        self.observe_mutations(expr, self.could_be_table_reference(expr));
      }
      false
    }
  }

  fn visit_stat_for_in(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let node = &*(node as *mut AstStatForIn);

      for &expr in node.values.iter() {
        self.observe_mutations(expr, true);
      }

      ast_stat_visit(node.body as *mut AstStat, self);
      false
    }
  }
}
