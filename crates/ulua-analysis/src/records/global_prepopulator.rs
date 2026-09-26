use core::ptr::{NonNull, from_mut};

use ulua_ast::records::{
  ast_expr_global::AstExprGlobal, ast_name::AstName, ast_stat_assign::AstStatAssign,
  ast_stat_function::AstStatFunction, ast_type::AstType, ast_type_pack::AstTypePack,
  ast_visitor::AstVisitor,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{data_flow_graph::DataFlowGraph, scope::Scope, type_arena::TypeArena};
#[derive(Debug, Clone)]
pub struct GlobalPrepopulator {
  pub global_scope: NonNull<Scope>,
  pub arena: NonNull<TypeArena>,
  pub dfg: NonNull<DataFlowGraph>,
  pub uninitialized_globals: DenseHashSet<AstName>,
}

impl AstVisitor for GlobalPrepopulator {
  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    self.visit_ast_expr_global(from_mut(node))
  }

  fn visit_type(&mut self, _node: &mut AstType) -> bool {
    true
  }

  fn visit_type_pack(&mut self, _node: &mut AstTypePack) -> bool {
    true
  }

  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    self.visit_ast_stat_assign(from_mut(node))
  }
  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_ast_stat_function(from_mut(node))
  }
}
