use core::ptr::NonNull;

use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  records::{constraint::Constraint, constraint_solver::ConstraintSolver},
  type_aliases::type_pack_id::TypePackId,
};

#[derive(Debug, Clone, Copy)]
pub struct MagicFunctionCallContext {
  pub solver: NonNull<ConstraintSolver>,
  pub constraint: NonNull<Constraint>,
  pub call_site: NonNull<AstExprCall>,
  pub arguments: TypePackId,
  pub result: TypePackId,
}
