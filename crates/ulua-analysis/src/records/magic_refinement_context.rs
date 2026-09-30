use alloc::vec::Vec;

use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{records::scope::Scope, type_aliases::type_id::TypeId};
#[derive(Debug, Clone)]
pub struct MagicRefinementContext {
  pub scope: *mut Scope,
  pub call_site: *const AstExprCall,
  pub discriminant_types: Vec<Option<TypeId>>,
}
