use core::ptr::NonNull;

use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  records::{builtin_types::BuiltinTypes, scope::Scope, type_checker_2::TypeChecker2},
  type_aliases::type_pack_id::TypePackId,
};

#[derive(Debug, Clone)]
pub struct MagicFunctionTypeCheckContext {
  pub(crate) typechecker: NonNull<TypeChecker2>,
  pub(crate) builtin_types: NonNull<BuiltinTypes>,
  pub(crate) call_site: *const AstExprCall,
  pub(crate) arguments: TypePackId,
  pub(crate) check_scope: NonNull<Scope>,
}
