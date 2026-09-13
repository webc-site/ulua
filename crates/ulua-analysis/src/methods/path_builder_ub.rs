use crate::{
  enums::type_field::TypeField, records::path_builder::PathBuilder,
  type_aliases::component::Component,
};

pub trait PathBuilderUb {
  fn ub(&mut self) -> &mut Self;
}

impl PathBuilderUb for PathBuilder {
  fn ub(&mut self) -> &mut Self {
    self
      .components
      .push(Component::TypeField(TypeField::UpperBound));
    self
  }
}
