use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_compound_assign::AstStatCompoundAssign, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatCompoundAssign {
  pub fn new(
    location: Location,
    op: AstExprBinaryOp,
    var: *mut AstExpr,
    value: *mut AstExpr,
  ) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      op,
      var,
      value,
    }
  }
}

pub fn ast_stat_compound_assign_ast_stat_compound_assign(
  location: Location,
  op: AstExprBinaryOp,
  var: *mut AstExpr,
  value: *mut AstExpr,
) -> AstStatCompoundAssign {
  AstStatCompoundAssign::new(location, op, var, value)
}
