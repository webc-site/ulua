use alloc::string::String;
use core::hash::{Hash, Hasher};

use ulua_ast::records::ast_expr_binary::AstExprBinaryOp;

use crate::enums::op_kind::OpKind;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CannotInferBinaryOperation {
  pub(crate) op: AstExprBinaryOp,
  pub(crate) suggested_to_annotate: Option<String>,
  pub(crate) kind: OpKind,
}

impl Hash for CannotInferBinaryOperation {
  fn hash<H: Hasher>(&self, state: &mut H) {
    (self.op as i32).hash(state);
    self.suggested_to_annotate.hash(state);
    self.kind.hash(state);
  }
}

impl CannotInferBinaryOperation {
  pub fn op(&self) -> AstExprBinaryOp {
    self.op
  }

  pub fn suggested_to_annotate(&self) -> Option<&str> {
    self.suggested_to_annotate.as_deref()
  }

  pub fn kind(&self) -> OpKind {
    self.kind
  }
}

impl CannotInferBinaryOperation {
  pub const fn new(
    op: AstExprBinaryOp,
    suggested_to_annotate: Option<String>,
    kind: OpKind,
  ) -> Self {
    Self {
      op,
      suggested_to_annotate,
      kind,
    }
  }
}
