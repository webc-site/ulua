use std::sync::Arc;

use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  records::{
    magic_function::MagicFunction, magic_function_call_context::MagicFunctionCallContext,
    scope::Scope, type_checker::TypeChecker, with_predicate::WithPredicate,
  },
  type_aliases::{old_solver_handler::OldSolverHandler, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct MagicFind {
  pub base: MagicFunction,
  pub(crate) handle_old_solver: OldSolverHandler,
  pub infer: fn(&MagicFunctionCallContext) -> bool,
}

impl MagicFind {
  pub fn handle_old_solver(
    &self,
    context: &mut TypeChecker,
    scope: &Arc<Scope>,
    call_site: &AstExprCall,
    old_result: WithPredicate<TypePackId>,
  ) -> Option<WithPredicate<TypePackId>> {
    (self.handle_old_solver)(context, scope, call_site, old_result)
  }
}
