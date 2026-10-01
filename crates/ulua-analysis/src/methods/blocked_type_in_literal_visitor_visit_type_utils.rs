use std::ptr::from_ref;

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_group::AstExprGroup},
  rtti::ast_node_is,
};

use crate::{
  functions::{follow_type, get_type, is_literal::is_literal},
  records::{
    blocked_type::BlockedType, blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor,
  },
};

impl BlockedTypeInLiteralVisitor<'_> {
  pub fn visit_ast_node(&mut self) -> bool {
    false
  }

  /// 形参全为受检引用（`ast_types`/`to_block` 由驱动方以独占借用注入，借用类型
  /// 即契约）。对应 C++ `bool BlockedTypeInLiteralVisitor::visit(AstExpr* e)`
  /// (`cpp/Analysis/src/TypeUtils.cpp:490`)。
  pub fn visit_ast_expr(&mut self, e: &AstExpr) -> bool {
    if let Some(&ty) = self.ast_types.find(&(from_ref(e))) {
      let followed = follow_type::follow(ty);
      if get_type::get::<BlockedType>(followed).is_some() {
        self.to_block.push(ty);
      }
    }
    is_literal(from_ref(e)) || ast_node_is::<AstExprGroup>(&e.base)
  }
}
