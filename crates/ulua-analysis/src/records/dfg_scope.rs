use crate::{
  enums::scope_type::ScopeType,
  type_aliases::{bindings::Bindings, props_data_flow_graph::Props},
};

#[derive(Debug, Clone)]
pub struct DfgScope {
  pub(crate) parent: *mut DfgScope,
  pub(crate) scope_type: ScopeType,
  pub(crate) bindings: Bindings,
  pub(crate) props: Props,
}
