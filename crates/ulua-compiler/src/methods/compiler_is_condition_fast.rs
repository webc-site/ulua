use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_group::AstExprGroup,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::{enums::type_constant_folding::Type, records::compiler::Compiler};

impl Compiler {
  pub fn is_condition_fast(&mut self, node: *mut AstExpr) -> bool {
    unsafe {
      if node.is_null() {
        return false;
      }

      let cv = self.constants.find(&node);

      if let Some(constant) = cv
        && constant.r#type != Type::Unknown
      {
        return true;
      }

      let binary = ast_node_as::<AstExprBinary>(node as *mut AstNode);
      if !binary.is_null() {
        match (*binary).op {
          AstExprBinaryOp::And
          | AstExprBinaryOp::Or
          | AstExprBinaryOp::CompareNe
          | AstExprBinaryOp::CompareEq
          | AstExprBinaryOp::CompareLt
          | AstExprBinaryOp::CompareLe
          | AstExprBinaryOp::CompareGt
          | AstExprBinaryOp::CompareGe => return true,
          _ => return false,
        }
      }

      let group = ast_node_as::<AstExprGroup>(node as *mut AstNode);
      if !group.is_null() {
        return self.is_condition_fast((*group).expr);
      }

      false
    }
  }
}
