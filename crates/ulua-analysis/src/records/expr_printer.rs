use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::def_id_control_flow_graph::DefId;
#[derive(Debug, Clone)]
pub struct ExprPrinter {
  pub(crate) use_defs: DenseHashMap<*mut AstExpr, DefId>,
  pub(crate) result: String,
}
