use core::ptr::null;

use crate::{
  enums::scope_type::ScopeType,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, dfg_scope::DfgScope, symbol::Symbol},
  type_aliases::{bindings::Bindings, props_data_flow_graph::Props},
};
impl DataFlowGraphBuilder {
  pub fn make_child_scope(&mut self, scope_type: ScopeType) -> *mut DfgScope {
    let parent_scope = self.current_scope();
    // C++ `new DfgScope{currentScope(), scopeType}` uses default member
    // inits: `bindings{Symbol{}}`, `props{nullptr}`.
    let new_scope = DfgScope {
      parent: parent_scope,
      scope_type,
      bindings: Bindings::new(Symbol::default()),
      props: Props::new(null()),
    };
    self.scopes.push(new_scope)
  }
}
