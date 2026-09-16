use ulua_ast::records::{ast_expr::AstExpr, ast_node::AstNode, ast_type::AstType};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    builtin_types::BuiltinTypes, expected_type_visitor::ExpectedTypeVisitor, scope::Scope,
    type_arena::TypeArena,
  },
  type_aliases::type_id::TypeId,
};

impl ExpectedTypeVisitor {
  pub fn new(
    ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
    ast_expected_types: *mut DenseHashMap<*const AstExpr, TypeId>,
    ast_resolved_types: *mut DenseHashMap<*const AstType, TypeId>,
    ast_overload_resolved_types: *mut DenseHashMap<*const AstNode, TypeId>,
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    root_scope: *mut Scope,
  ) -> Self {
    Self {
      ast_types,
      ast_expected_types,
      ast_resolved_types,
      ast_overload_resolved_types,
      arena,
      builtin_types,
      root_scope,
    }
  }
}
