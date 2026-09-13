extern crate alloc;

use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{records::expr_printer::ExprPrinter, type_aliases::definition::Definition};

/// # Safety
/// `expr` 必须指向有效 AstExpr 节点。
pub unsafe fn dump_expr(
  expr: *mut AstExpr,
  use_defs: &DenseHashMap<*mut AstExpr, *mut Definition>,
) -> String {
  let mut printer = ExprPrinter::new(use_defs.clone());
  // SAFETY: expr 指向有效 AstExpr 节点
  unsafe { printer.visit_ast_expr(expr) };
  printer.result
}
