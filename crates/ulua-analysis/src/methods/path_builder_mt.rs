use crate::{
  enums::type_field::TypeField, records::path_builder::PathBuilder,
  type_aliases::component::Component,
};

pub trait PathBuilderMt {
  fn mt(&mut self) -> &mut Self;
}

impl PathBuilderMt for PathBuilder {
  fn mt(&mut self) -> &mut Self {
    self
      .components
      .push(Component::TypeField(TypeField::Metatable));
    self
  }
}
