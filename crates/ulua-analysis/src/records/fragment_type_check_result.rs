use alloc::vec::Vec;

use ulua_ast::records::ast_node::AstNode;

use crate::type_aliases::{module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr};
#[derive(Debug, Clone)]
pub struct FragmentTypeCheckResult {
  pub incremental_module: Option<ModulePtr>,
  pub fresh_scope: ScopePtr,
  pub ancestry: Vec<*mut AstNode>,
}
