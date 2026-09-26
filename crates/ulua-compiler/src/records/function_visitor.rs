use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_stat::AstStat,
    ast_stat_type_function::AstStatTypeFunction, ast_visitor::AstVisitor,
  },
  visit::ast_stat_visit,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::node::Node;

#[derive(Debug)]
pub(crate) struct FunctionVisitor<'a> {
  pub(crate) functions: &'a mut Vec<Node<AstExprFunction>>,
  pub(crate) has_types: bool,
  pub(crate) has_native_function: bool,
}

impl<'a> FunctionVisitor<'a> {
  pub fn new(functions: &'a mut Vec<Node<AstExprFunction>>) -> Self {
    functions.reserve(16);
    Self {
      functions,
      has_types: false,
      has_native_function: false,
    }
  }
}

impl<'a> AstVisitor for FunctionVisitor<'a> {
  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    // Safety: node.body 为 parser 保证非空存活的函数体 AstStatBlock，向上转
    // AstStat 依 repr(C) 前缀重合合法；FunctionVisitor 只收集指针、不写 AST。
    unsafe {
      ast_stat_visit(node.body.as_ptr().cast::<AstStat>(), self);
    }

    self.has_types |= node.args.iter_nodes().any(|arg| !arg.annotation.is_null());

    // 确保编译本函数所用的一切子函数都已先入 vector（先序不变式）
    LUAU_ASSERT!(self.functions.iter().all(|&f| f != Node::from_mut(node)));
    self.functions.push(Node::from_mut(node));

    if !self.has_native_function && node.has_native_attribute() {
      self.has_native_function = true;
    }

    false
  }

  fn visit_stat_type_function(&mut self, _node: &mut AstStatTypeFunction) -> bool {
    false
  }
}
