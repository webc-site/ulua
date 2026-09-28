use ulua_ast::records::ast_expr::AstExpr;

use crate::records::{constant::Constant, node::Node};

#[derive(Debug, Clone)]
pub struct ExprConstantChange {
  pub(crate) key: Node<AstExpr>,
  pub(crate) old_value: Constant,
  /// 供集成测试（tests/constant_folding.rs）断言 change log 语义
  pub was_absent: bool,
}
