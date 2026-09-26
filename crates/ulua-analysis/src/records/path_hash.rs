use ulua_common::collections::fast_hash;

use crate::{records::path::Path, type_aliases::component::Component};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct PathHash;

impl PathHash {
  #[inline]
  pub fn hash_component(&self, component: &Component) -> usize {
    match component {
      Component::Property(prop) => fast_hash(prop.name()) ^ (prop.is_read() as usize),
      Component::Index(idx) => idx.index,
      Component::TypeField(field) => *field as usize,
      Component::PackField(field) => *field as usize,
      Component::PackSlice(slice) => slice.start_index,
      Component::Reduction(reduction) => fast_hash(&reduction.result_type),
      Component::GenericPackMapping(mapping) => fast_hash(&mapping.mapped_type),
    }
  }

  #[inline]
  pub fn operator_component(&self, component: &Component) -> usize {
    self.hash_component(component)
  }

  #[inline]
  pub fn operator_call(&self, component: &Component) -> usize {
    self.hash_component(component)
  }

  pub fn hash_path(&self, path: &Path) -> usize {
    path
      .components
      .iter()
      .fold(0, |hash, c| hash ^ self.hash_component(c))
  }

  #[inline]
  pub fn operator_call_6(&self, path: &Path) -> usize {
    self.hash_path(path)
  }
}
