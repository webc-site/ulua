use crate::{
  records::{pack_slice::PackSlice, path_builder::PathBuilder},
  type_aliases::component::Component,
};

impl PathBuilder {
  pub fn pack_slice(&mut self, start_index: usize) -> &mut Self {
    self
      .components
      .push(Component::PackSlice(PackSlice { start_index }));
    self
  }
}
