use crate::{
  enums::pack_field::PackField, records::path_builder::PathBuilder,
  type_aliases::component::Component,
};

pub trait PathBuilderTail {
  fn tail(&mut self) -> &mut Self;
}

impl PathBuilderTail for PathBuilder {
  fn tail(&mut self) -> &mut Self {
    self.components.push(Component::PackField(PackField::Tail));
    self
  }
}
