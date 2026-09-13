use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  records::{type_checker::TypeChecker, with_predicate::WithPredicate},
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};

pub fn magic_pcall_handle_old_solver(
  _typechecker: &mut TypeChecker,
  _scope: &ScopePtr,
  _expr: &AstExprCall,
  _with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  // pcall() is only magic in the new solver.
  None
}
