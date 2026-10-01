extern crate alloc;

use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{records::expr_printer::ExprPrinter, type_aliases::def_id_control_flow_graph::DefId};

pub fn dump_expr(expr: &AstExpr, use_defs: &DenseHashMap<*mut AstExpr, DefId>) -> String {
  let mut printer = ExprPrinter::new(use_defs.clone());
  printer.visit_ast_expr(expr);
  printer.result
}
