use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_table::DenseDefault;

use crate::records::node::Node;

#[derive(Debug, Clone, Copy, Default)]
pub struct Variable {
  /// cpp `AstExpr* init = nullptr`：变量初值表达式，由 trackValues 填充；
  /// `None` 表示无初值（cpp null 哨兵 → Option）。
  pub(crate) init: Option<Node<AstExpr>>,
  pub(crate) written: bool, // is the variable ever assigned to? filled by trackValues
  pub(crate) constant: bool, // is the variable's value a compile-time constant? filled by constantFold
}

impl DenseDefault for Variable {
  fn dense_default() -> Self {
    Self::default()
  }
}
