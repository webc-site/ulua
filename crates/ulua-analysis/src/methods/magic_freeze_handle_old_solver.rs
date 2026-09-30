use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  records::{type_checker::TypeChecker, with_predicate::WithPredicate},
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};

pub fn magic_freeze_handle_old_solver(
  _typechecker: &mut TypeChecker,
  _scope: &ScopePtr,
  _call_site: &AstExprCall,
  _old_result: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  None
}
