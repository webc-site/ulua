use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_type::AstType,
  },
  visit,
};
use ulua_common::enums::luau_bytecode_type::{LBC_TYPE_NUMBER, LBC_TYPE_VECTOR};

use crate::records::type_map_visitor::TypeMapVisitor;

impl TypeMapVisitor<'_> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_unary(&mut self, node: *mut AstExprUnary) -> bool {
    unsafe {
      if node.is_null() {
        return false;
      }

      let node_ref = &*node;
      let expr = node_ref.expr;

      visit::ast_expr_visit(expr, self);

      match node_ref.op {
        AstExprUnaryOp::Not => {
          self.record_resolved_type_ast_expr_ast_type(
            node as *mut AstExpr,
            &self.builtin_types.boolean_type as *const _ as *const AstType,
          );
        }
        AstExprUnaryOp::Minus => {
          let type_ptr = self.resolved_exprs.find(&expr);
          let bc_type_ptr = self.expr_types.find(&expr);

          if let (Some(&ty), Some(&bc_ty)) = (type_ptr, bc_type_ptr)
            && (bc_ty == LBC_TYPE_VECTOR || bc_ty == LBC_TYPE_NUMBER)
          {
            self.record_resolved_type_ast_expr_ast_type(node as *mut AstExpr, ty);
          }
        }
        AstExprUnaryOp::Len => {
          self.record_resolved_type_ast_expr_ast_type(
            node as *mut AstExpr,
            &self.builtin_types.number_type as *const _ as *const AstType,
          );
        }
      }

      false
    }
  }
}
