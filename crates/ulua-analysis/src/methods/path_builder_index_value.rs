use crate::{
  enums::type_field::TypeField, records::path_builder::PathBuilder,
  type_aliases::component::Component,
};

impl PathBuilder {
  pub fn index_value(&mut self) -> &mut Self {
    self
      .components
      .push(Component::TypeField(TypeField::IndexResult));
    self
  }
}
