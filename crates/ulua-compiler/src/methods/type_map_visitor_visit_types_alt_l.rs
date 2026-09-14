use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_type::AstType,
  },
  visit::ast_expr_visit,
};
use ulua_common::enums::luau_bytecode_type::{LBC_TYPE_NUMBER, LBC_TYPE_VECTOR};

use crate::records::type_map_visitor::TypeMapVisitor;

impl TypeMapVisitor<'_> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_binary(&mut self, node: *mut AstExprBinary) -> bool {
    unsafe {
      if node.is_null() {
        return false;
      }

      let left = (*node).left;
      let right = (*node).right;

      ast_expr_visit(left, self);
      ast_expr_visit(right, self);

      let op = (*node).op;

      if op == AstExprBinaryOp::CompareNe
        || op == AstExprBinaryOp::CompareEq
        || op == AstExprBinaryOp::CompareLt
        || op == AstExprBinaryOp::CompareLe
        || op == AstExprBinaryOp::CompareGt
        || op == AstExprBinaryOp::CompareGe
      {
        self.record_resolved_type_ast_expr_ast_type(
          node as *mut AstExpr,
          &self.builtin_types.boolean_type as *const _ as *const AstType,
        );
        return false;
      }

      if op == AstExprBinaryOp::Concat || op == AstExprBinaryOp::And || op == AstExprBinaryOp::Or {
        return false;
      }

      let left_type_ptr = self.resolved_exprs.find(&left);
      let left_bc_type_ptr = self.expr_types.find(&left);

      if left_type_ptr.is_none() || left_bc_type_ptr.is_none() {
        return false;
      }

      let right_type_ptr = self.resolved_exprs.find(&right);
      let right_bc_type_ptr = self.expr_types.find(&right);

      if right_type_ptr.is_none() || right_bc_type_ptr.is_none() {
        return false;
      }

      let left_bc_type = *left_bc_type_ptr.unwrap();
      let right_bc_type = *right_bc_type_ptr.unwrap();

      if left_bc_type == LBC_TYPE_VECTOR {
        self.record_resolved_type_ast_expr_ast_type(node as *mut AstExpr, *left_type_ptr.unwrap());
      } else if right_bc_type == LBC_TYPE_VECTOR {
        self.record_resolved_type_ast_expr_ast_type(node as *mut AstExpr, *right_type_ptr.unwrap());
      } else if left_bc_type == LBC_TYPE_NUMBER && right_bc_type == LBC_TYPE_NUMBER {
        self.record_resolved_type_ast_expr_ast_type(node as *mut AstExpr, *left_type_ptr.unwrap());
      }

      false
    }
  }
}
