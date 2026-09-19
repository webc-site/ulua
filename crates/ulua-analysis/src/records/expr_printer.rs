use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::sym_def::SymDef;
#[derive(Debug, Clone)]
pub struct ExprPrinter {
  pub(crate) use_defs: DenseHashMap<*mut AstExpr, *mut SymDef>,
  pub(crate) result: String,
}
