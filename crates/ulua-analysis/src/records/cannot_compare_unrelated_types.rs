use core::hash::{Hash, Hasher};

use ulua_ast::records::ast_expr_binary::AstExprBinaryOp;

use crate::type_aliases::type_id::TypeId;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CannotCompareUnrelatedTypes {
  pub(crate) left: TypeId,
  pub(crate) right: TypeId,
  pub(crate) op: AstExprBinaryOp,
}

impl Hash for CannotCompareUnrelatedTypes {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.left.hash(state);
    self.right.hash(state);
    (self.op as i32).hash(state);
  }
}
