use alloc::vec::Vec;
use core::ffi::c_void;

use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName, ast_visitor::AstVisitor};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{data_flow_graph::DataFlowGraph, def::Def, symbol::Symbol},
  type_aliases::name_type::Name,
};
#[derive(Debug, Clone)]
pub struct UsageFinder {
  pub dfg: *mut DataFlowGraph,
  pub declared_aliases: DenseHashSet<Name>,
  pub local_bindings_referenced: Vec<(*const Def, *mut AstLocal)>,
  pub mentioned_defs: DenseHashSet<*const Def>,
  pub referenced_bindings: Vec<Name>,
  pub referenced_imported_bindings: Vec<(Name, Name)>,
  pub global_defs_to_pre_populate: Vec<(AstName, *const Def)>,
  pub global_functions_referenced: Vec<AstName>,
  pub symbols_to_refine: Vec<(*const Def, Symbol)>,
}

impl AstVisitor for UsageFinder {
  fn visit_expr_constant_string(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_constant_string(node as *mut _)
  }
  fn visit_type(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type(node as *mut _)
  }
  fn visit_type_pack(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_pack(node as *mut _)
  }
  fn visit_stat_type_alias(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_type_alias(node as *mut _)
  }
  fn visit_type_reference(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_reference(node as *mut _)
  }
  fn visit_expr(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr(node as *mut _)
  }
  fn visit_expr_global(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_global(node as *mut _)
  }
  fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_function(node as *mut _)
  }
}
