use crate::{
  enums::variant::Variant,
  records::{index::Index, path_builder::PathBuilder},
  type_aliases::component::Component,
};
pub trait PathBuilderIndex {
  fn index(&mut self, i: usize) -> &mut Self;
}

impl PathBuilderIndex for PathBuilder {
  fn index(&mut self, i: usize) -> &mut Self {
    self.components.push(Component::Index(Index {
      index: i,
      variant: Variant::Pack,
    }));
    self
  }
}
