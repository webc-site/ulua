use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{records::expr_printer::ExprPrinter, type_aliases::def_id_control_flow_graph::DefId};
impl ExprPrinter {
  pub fn new(use_defs: DenseHashMap<*mut AstExpr, DefId>) -> Self {
    Self {
      use_defs,
      result: String::new(),
    }
  }
}
