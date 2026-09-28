use ulua_ast::records::{ast_expr::AstExpr, ast_local::AstLocal};

use crate::records::{constant::Constant, node::Node};

#[derive(Debug, Clone)]
pub(crate) struct InlineArg {
  pub(crate) local: Node<AstLocal>,
  pub(crate) reg: u8,
  pub(crate) value: Constant,
  pub(crate) allocpc: u32,
  /// cpp `AstExpr* init = nullptr`：实参对应的初值表达式；`None` 表示无初值
  /// （cpp null 哨兵 → Option，与 Variable.init 同族）。
  pub(crate) init: Option<Node<AstExpr>>,
}
