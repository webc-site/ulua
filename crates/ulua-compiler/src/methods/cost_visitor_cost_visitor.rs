use core::ptr::null_mut;

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_call::AstExprCall};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{constant::Constant, cost::Cost, cost_visitor::CostVisitor};

impl CostVisitor {
  pub fn new(
    builtins: &DenseHashMap<*mut AstExprCall, i32>,
    constants: &DenseHashMap<*mut AstExpr, Constant>,
  ) -> Self {
    Self {
      builtins: builtins as *const DenseHashMap<*mut AstExprCall, i32>,
      constants: constants as *const DenseHashMap<*mut AstExpr, Constant>,
      vars: DenseHashMap::new(null_mut()),
      result: Cost::default(),
    }
  }
}
