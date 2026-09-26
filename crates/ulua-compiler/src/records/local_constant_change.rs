use ulua_ast::records::ast_local::AstLocal;

use crate::records::{constant::Constant, node::Node};

#[derive(Debug, Clone)]
pub struct LocalConstantChange {
  pub(crate) key: Node<AstLocal>,
  pub(crate) old_value: Constant,
  pub(crate) was_absent: bool,
}
