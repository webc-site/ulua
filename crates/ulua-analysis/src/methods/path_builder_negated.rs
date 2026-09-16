use crate::{
  enums::type_field::TypeField, records::path_builder::PathBuilder,
  type_aliases::component::Component,
};

pub trait PathBuilderNegated {
  fn negated(&mut self) -> &mut Self;
}

impl PathBuilderNegated for PathBuilder {
  fn negated(&mut self) -> &mut Self {
    self
      .components
      .push(Component::TypeField(TypeField::Negated));
    self
  }
}
