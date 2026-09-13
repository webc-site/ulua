use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::definition::Definition;
#[derive(Debug, Clone)]
pub struct ExprPrinter {
  pub(crate) use_defs: DenseHashMap<*mut AstExpr, *mut Definition>,
  pub(crate) result: String,
}
