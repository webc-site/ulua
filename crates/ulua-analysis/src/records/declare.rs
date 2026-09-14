use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::type_aliases::def_id_control_flow_graph::DefId;

#[derive(Debug, Clone)]
pub struct Declare {
  pub def: DefId,
  pub source: *mut AstStatLocal,
}

impl Declare {
  pub fn new(def: DefId, source: *mut AstStatLocal) -> Self {
    Self { def, source }
  }
}
