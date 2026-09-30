use alloc::vec::Vec;
use core::option::Option;

use ulua_ast::records::{ast_expr_call::AstExprCall, ast_node::AstNode};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

#[derive(Debug, Clone)]
pub struct FunctionCallConstraint {
  pub(crate) fn_type: TypeId,
  pub(crate) args_pack: TypePackId,
  pub(crate) result: TypePackId,
  pub(crate) call_site: *mut AstExprCall,
  pub(crate) discriminant_types: Vec<Option<TypeId>>,
  pub(crate) type_arguments: Vec<TypeId>,
  pub(crate) type_pack_arguments: Vec<TypePackId>,
  pub(crate) ast_overload_resolved_types: *mut DenseHashMap<*const AstNode, TypeId>,
}
