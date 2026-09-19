use crate::{
  enums::type_field::TypeField, records::path_builder::PathBuilder,
  type_aliases::component::Component,
};

pub trait PathBuilderLb {
  fn lb(&mut self) -> &mut Self;
}

impl PathBuilderLb for PathBuilder {
  fn lb(&mut self) -> &mut Self {
    self
      .components
      .push(Component::TypeField(TypeField::LowerBound));
    self
  }
}
