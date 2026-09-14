use core::ffi::c_void;

use ulua_ast::records::{
  ast_expr_function::AstExprFunction, ast_local::AstLocal, ast_name::AstName,
  ast_stat_assign::AstStatAssign, ast_stat_class::AstStatClass,
  ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_function::AstStatFunction,
  ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
  ast_visitor::AstVisitor,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{enums::global::Global, records::variable::Variable};

#[derive(Debug, Clone)]
pub struct ValueVisitor {
  pub(crate) globals: DenseHashMap<AstName, Global>,
  pub(crate) variables: DenseHashMap<*mut AstLocal, Variable>,
  pub(crate) class_locals: DenseHashMap<AstName, *mut AstLocal>,
}

// Wire the generic `AstVisitor` dispatch (used by `var->visit(this)` /
// `dispatch_node`) to the concrete `visit_ast_*` overloads. Only the statements
// the C++ `ValueVisitor` overrides are listed; everything else uses the trait's
// default (recurse) behavior.
impl AstVisitor for ValueVisitor {
  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local(node as *mut AstStatLocal)
  }

  fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_assign(node as *mut AstStatAssign)
  }

  fn visit_stat_compound_assign(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_compound_assign(node as *mut AstStatCompoundAssign)
  }

  fn visit_stat_local_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local_function(node as *mut AstStatLocalFunction)
  }

  fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_function(node as *mut AstStatFunction)
  }

  fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_function(node as *mut AstExprFunction)
  }

  fn visit_stat_class(&mut self, node: *mut c_void) -> bool {
    unsafe { self.visit_ast_stat_class(node as *mut AstStatClass) }
  }
}
