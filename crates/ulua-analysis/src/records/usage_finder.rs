use alloc::vec::Vec;
use core::ptr::from_mut;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
  ast_expr_global::AstExprGlobal, ast_local::AstLocal, ast_name::AstName,
  ast_stat_function::AstStatFunction, ast_stat_type_alias::AstStatTypeAlias, ast_type::AstType,
  ast_type_pack::AstTypePack, ast_type_reference::AstTypeReference, ast_visitor::AstVisitor,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{data_flow_graph::DataFlowGraph, symbol::Symbol},
  type_aliases::{def_id_def::DefId, name_type::Name},
};
#[derive(Debug, Clone)]
pub struct UsageFinder {
  pub dfg: *mut DataFlowGraph,
  pub declared_aliases: DenseHashSet<Name>,
  pub local_bindings_referenced: Vec<(DefId, *mut AstLocal)>,
  pub mentioned_defs: DenseHashSet<DefId>,
  pub referenced_bindings: Vec<Name>,
  pub referenced_imported_bindings: Vec<(Name, Name)>,
  pub global_defs_to_pre_populate: Vec<(AstName, DefId)>,
  pub global_functions_referenced: Vec<AstName>,
  pub symbols_to_refine: Vec<(DefId, Symbol)>,
}

impl AstVisitor for UsageFinder {
  fn visit_expr_constant_string(&mut self, node: &mut AstExprConstantString) -> bool {
    self.visit_ast_expr_constant_string(from_mut(node))
  }
  fn visit_type(&mut self, _node: &mut AstType) -> bool {
    self.visit_ast_type()
  }
  fn visit_type_pack(&mut self, _node: &mut AstTypePack) -> bool {
    self.visit_ast_type_pack()
  }
  fn visit_stat_type_alias(&mut self, node: &mut AstStatTypeAlias) -> bool {
    self.visit_ast_stat_type_alias(from_mut(node))
  }
  fn visit_type_reference(&mut self, node: &mut AstTypeReference) -> bool {
    self.visit_ast_type_reference(from_mut(node))
  }
  fn visit_expr(&mut self, node: &mut AstExpr) -> bool {
    self.visit_ast_expr(from_mut(node))
  }
  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    self.visit_ast_expr_global(from_mut(node))
  }
  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_ast_stat_function(from_mut(node))
  }
}
