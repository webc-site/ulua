use ulua_ast::records::ast_stat_assign::AstStatAssign;

use crate::type_aliases::def_id_control_flow_graph::DefId;

#[derive(Debug, Clone)]
pub struct Assign {
  pub def: DefId,
  pub source: *mut AstStatAssign,
}

impl Assign {
  pub fn new(def: DefId, source: *mut AstStatAssign) -> Self {
    Self { def, source }
  }
}
