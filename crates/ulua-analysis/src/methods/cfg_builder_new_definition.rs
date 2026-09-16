use crate::{
  records::{cfg_builder::CfgBuilder, symbol::Symbol},
  type_aliases::def_id_control_flow_graph::DefId,
};

impl CfgBuilder {
  pub fn new_definition(&mut self, sym: Symbol) -> DefId {
    let version = self.next_version_index(sym.clone());
    let allocator = unsafe { &mut *self.allocator };
    allocator.new_definition(sym, version)
  }
}
