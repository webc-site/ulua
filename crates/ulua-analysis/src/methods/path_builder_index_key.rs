use crate::{
  enums::type_field::TypeField, records::path_builder::PathBuilder,
  type_aliases::component::Component,
};

impl PathBuilder {
  pub fn index_key(&mut self) -> &mut Self {
    self
      .components
      .push(Component::TypeField(TypeField::IndexLookup));
    self
  }
}
