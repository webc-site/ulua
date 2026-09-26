use alloc::vec::Vec;

use ulua_ast::records::{ast_node::AstNode, location::Location};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{records::module_info::ModuleInfo, type_aliases::module_name_type::ModuleName};
#[derive(Debug, Clone)]
pub struct RequireTraceResult {
  pub exprs: DenseHashMap<*mut AstNode, ModuleInfo>,
  pub require_list: Vec<(ModuleName, Location)>,
}
