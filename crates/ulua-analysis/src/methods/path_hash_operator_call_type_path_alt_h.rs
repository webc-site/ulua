use crate::{records::path_hash::PathHash, type_aliases::component::Component};

impl PathHash {
  pub fn operator_call(&self, component: &Component) -> usize {
    self.operator_component(component)
  }
}
