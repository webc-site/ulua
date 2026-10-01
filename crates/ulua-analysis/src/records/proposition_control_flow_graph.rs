use alloc::string::String;

use crate::type_aliases::def_id_control_flow_graph::DefId;

#[derive(Debug, Clone)]
pub struct Proposition {
  pub ptr: DefId,
  pub r#type: Option<String>,
  pub is_typeof: bool,
  pub sense: bool,
}
